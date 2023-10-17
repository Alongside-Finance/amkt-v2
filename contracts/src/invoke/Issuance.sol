pragma solidity =0.8.18;

import {VerifiableAddressArray} from "src/lib/VArray.sol";
import {IIndexToken} from "src/interfaces/IIndexToken.sol";
import {TokenInfo} from "src/Common.sol";
import {IVault} from "src/interfaces/IVault.sol";
import {SCALAR, fmul, fdiv} from "src/lib/FixedPoint.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IIssuance} from "src/interfaces/IIssuance.sol";

contract Issuance is IIssuance {
    using VerifiableAddressArray for VerifiableAddressArray.VerifiableArray;
    using SafeERC20 for IERC20;

    IVault public immutable vault;

    uint256 public reentrancyLock = 1;

    modifier invariantCheck() {
        _;
        vault.invariantCheck();
    }

    modifier ReentrancyGuard() {
        if (reentrancyLock > 1) revert IssuanceReentrant();
        reentrancyLock = 2;
        _;
        reentrancyLock = 1;
    }

    constructor(address _vault) {
        vault = IVault(_vault);
    }

    /// @notice Issue index tokens
    /// @param amount The amount of index tokens to issue
    /// @dev requires approval of underlying tokens
    /// @dev reentrancy guard in case callback in tokens
    function issue(uint256 amount) external invariantCheck ReentrancyGuard {
        TokenInfo[] memory tokens = vault.virtualUnits();

        if (tokens.length == 0) revert IssuanceNoTokens();

        for (uint256 i; i < tokens.length; ) {
            uint256 underlyingAmount = fmul(tokens[i].units + 1, amount) + 1;

            IERC20(tokens[i].token).safeTransferFrom(
                msg.sender,
                address(vault),
                underlyingAmount
            );

            unchecked {
                ++i;
            }
        }

        vault.invokeMint(msg.sender, amount);
    }

    /// @notice Redeem index tokens
    /// @param amount The amount of index tokens to redeem
    /// @dev requies approval of index token
    /// @dev reentrancy guard in case callback in tokens
    function redeem(uint256 amount) external invariantCheck ReentrancyGuard {
        TokenInfo[] memory tokens = vault.virtualUnits();

        if (tokens.length == 0) revert IssuanceNoTokens();

        IVault.InvokeERC20Args[] memory args = new IVault.InvokeERC20Args[](
            tokens.length
        );

        for (uint256 i; i < tokens.length; ) {
            uint256 underlyingAmount = fmul(tokens[i].units, amount);

            args[i] = IVault.InvokeERC20Args({
                token: tokens[i].token,
                to: msg.sender,
                amount: underlyingAmount
            });

            unchecked {
                ++i;
            }
        }

        vault.invokeBurn(msg.sender, amount);

        vault.invokeERC20s(args);
    }
}
