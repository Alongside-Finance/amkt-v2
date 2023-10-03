pragma solidity =0.8.15;

import {VerifiableAddressArray} from "../lib/VArray.sol";
import {IIndexToken} from "../interfaces/IIndexToken.sol";
import {TokenInfo} from "../Common.sol";
import {IVault} from "../interfaces/IVault.sol";
import {SCALAR, fmul, fdiv} from "../lib/FixedPoint.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

contract Issuance {
    error IssuanceReentrant();
    error IssuanceNotFeeRecipient();
    error IssuanceFeeTooEarly();

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

        require(tokens.length > 0, "No tokens in vault");

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

        require(tokens.length > 0, "No tokens in vault");

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

    ///////////////////////// INFLATION /////////////////////////

    /// @notice Try to accrue inflation
    function tryInflation() external invariantCheck {
        if (msg.sender != vault.feeRecipient())
            revert IssuanceNotFeeRecipient();
        if (block.timestamp < vault.lastKnownTimestamp() + 1 days)
            revert IssuanceFeeTooEarly();
        uint256 startingSupply = vault.indexToken().totalSupply();
        uint256 timestampDiff = block.timestamp - vault.lastKnownTimestamp();
        uint256 feeMultiplier = timestampDiff * vault.feeScaled();
        uint256 inflation = fdiv(startingSupply, feeMultiplier) -
            startingSupply;
        if (inflation == 0) revert IssuanceFeeTooEarly();

        TokenInfo[] memory tokens = vault.virtualUnits();

        IVault.SetNominalArgs[] memory args = new IVault.SetNominalArgs[](
            tokens.length
        );

        for (uint256 i; i < tokens.length; ) {
            args[i] = IVault.SetNominalArgs({
                token: tokens[i].token,
                virtualUnits: fmul(tokens[i].units, feeMultiplier)
            });

            unchecked {
                ++i;
            }
        }

        vault.invokeSetNominals(args);

        vault.invokeMintFee(inflation);
    }
}
