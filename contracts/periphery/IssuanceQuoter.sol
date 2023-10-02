pragma solidity =0.8.18;
import {IVault} from "src/interfaces/IVault.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {TokenInfo} from "src/Common.sol";

contract IssuanceQuoter {
    IVault vault;

    constructor(address _vault) {
        vault = IVault(_vault);
    }

    /// @notice Quote the amount of underlying tokens needed to issue index tokens
    /// @param amount The amount of index tokens to issue
    /// @return tokens The underlying tokens and amounts needed to issue index tokens
    /// @dev does not support tryInflation() yet
    function quoteIssue(
        uint256 amount
    ) external view returns (TokenInfo[] memory) {
        TokenInfo[] memory tokens = vault.virtualUnits();

        require(tokens.length > 0, "No tokens in vault");

        for (uint256 i; i < tokens.length; i++) {
            uint256 underlyingAmount = fmul(tokens[i].units + 1, amount) + 1;

            tokens[i].units = underlyingAmount;
        }

        return tokens;
    }
}
