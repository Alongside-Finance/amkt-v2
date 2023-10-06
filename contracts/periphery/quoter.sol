pragma solidity =0.8.18;
import {IVault} from "src/interfaces/IVault.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {TokenInfo} from "src/Common.sol";
import {Bounty} from "src/invoke/Bounty.sol";

contract Quoter {
    IVault immutable vault;

    constructor(address _vault) {
        vault = IVault(_vault);
    }

    /// @notice Quote the amount of underlying tokens needed to issue index tokens
    /// @param amount The amount of index tokens to issue
    /// @return tokens The underlying tokens and amounts needed to issue index tokens
    /// @dev Does not check for revert cases
    function quoteIssue(
        uint256 amount
    ) external view returns (TokenInfo[] memory) {
        TokenInfo[] memory tokens = vault.virtualUnits();

        for (uint256 i; i < tokens.length; i++) {
            uint256 underlyingAmount = fmul(tokens[i].units + 1, amount) + 1;

            tokens[i].units = underlyingAmount;
        }

        return tokens;
    }

    /// @notice Quote the amount of underlying tokens to be be received upon redemption of index tokens
    /// @param amount The amount of index tokens to redeem
    /// @return tokens The underlying tokens and amounts to be received upon redemption of index tokens
    /// @dev Does not check for revert cases
    function quoteRedeem(
        uint256 amount
    ) external view returns (TokenInfo[] memory) {
        TokenInfo[] memory tokens = vault.virtualUnits();

        for (uint256 i; i < tokens.length; i++) {
            uint256 underlyingAmount = fmul(tokens[i].units, amount);

            tokens[i].units = underlyingAmount;
        }

        return tokens;
    }

    /// @notice Quote a bounty, returns the ins and outs
    /// @dev The units in the bounty are the target units, i.e. amount of units per 1e18 amkt
    /// @dev Does not check for revert cases
    /// @param bounty The bounty to quote
    /// @param tokenSupply The total supply of the index token to quote for
    /// @return outs The tokens and amounts to send to the bounty fulfiller
    /// @return ins The tokens and amounts to receive from the bounty fulfiller
    function quoteFulfillBounty(
        Bounty calldata bounty,
        uint256 tokenSupply
    ) external view returns (TokenInfo[] memory outs, TokenInfo[] memory ins) {
        TokenInfo[] memory targets = bounty.infos;

        outs = new TokenInfo[](bounty.infos.length);

        ins = new TokenInfo[](bounty.infos.length);

        uint256 lenOuts;
        uint256 lenIns;

        for (uint256 i; i < bounty.infos.length; i++) {
            address token = bounty.infos[i].token;

            uint256 targetUnits = bounty.infos[i].units;

            uint256 virtualUnits = vault.virtualUnits(token);

            if (virtualUnits > targetUnits) {
                outs[lenOuts] = TokenInfo(
                    token,
                    fmul(virtualUnits - targetUnits, tokenSupply)
                );

                unchecked {
                    lenOuts++;
                }
            } else if (targetUnits > virtualUnits) {
                ins[lenIns] = TokenInfo(
                    token,
                    fmul(targetUnits - virtualUnits + 1, tokenSupply) + 1
                );

                unchecked {
                    lenIns++;
                }
            } else {
                continue;
            }
        }

        assembly {
            mstore(outs, lenOuts)
            mstore(ins, lenIns)
        }

        return (outs, ins);
    }
}
