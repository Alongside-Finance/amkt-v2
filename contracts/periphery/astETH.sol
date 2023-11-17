import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Ownable2Step} from "@openzeppelin/contracts/access/Ownable2Step.sol";

contract astETH is ERC20, Ownable2Step {
    using SafeERC20 for IERC20;

    IERC20 public immutable stETH;
    address public feeRecipient;

    constructor(
        IERC20 _stETH,
        address _owner,
        address _feeRecipient
    ) ERC20("Alongside stETH", "astETH") {
        stETH = _stETH;
        feeRecipient = _feeRecipient;
        _transferOwnership(_owner);
    }

    ///////////////////////// PERMISSIONLESS /////////////////////////

    /// @notice Deposit stETH to receive astETH
    /// @param  amount stETH amount to deposit
    /// @dev    Must pass invariant check
    function deposit(uint256 amount) external {
        stETH.safeTransferFrom(msg.sender, address(this), amount);
        _mint(msg.sender, amount);
        _invariantCheck();
    }

    /// @notice Burn astETH to receive stETH
    /// @param  amount astETH amount to burn
    /// @dev    Must pass invariant check
    function withdraw(uint256 amount) external {
        _burn(msg.sender, amount);
        stETH.safeTransfer(msg.sender, amount);
        _invariantCheck();
    }

    /// @notice Transfers excess stETH in the contract to the fee recipient
    /// @dev    Must pass invariant check
    function collectFee() external {
        uint256 stETHBalance = stETH.balanceOf(address(this));
        uint256 feeToCollect = stETHBalance - totalSupply();
        stETH.safeTransfer(feeRecipient, feeToCollect);
        _invariantCheck();
    }

    /// @notice Burn astETH to receive adjusted stETH amounts,
    ///         when stETH balance in the contract is less than the total supply.
    /// @param  amount astETH amount to burn
    /// @dev    Must fail invariant check
    function emergencyWithdraw(uint256 amount) external {
        uint256 stETHBalance = stETH.balanceOf(address(this));
        uint256 startingSupply = totalSupply();
        require(stETHBalance < startingSupply, "Invariant check passed");
        uint256 adjustedAmount = (amount * stETHBalance) / startingSupply;
        _burn(msg.sender, amount);
        stETH.safeTransfer(msg.sender, adjustedAmount);
    }

    ///////////////////////// OWNER /////////////////////////

    /// @param _feeRecipient address of new fee recipient
    function setFeeRecipient(address _feeRecipient) external onlyOwner {
        feeRecipient = _feeRecipient;
    }

    ///////////////////////// INTERNAL /////////////////////////

    /// @dev We assume that this invariant must hold in order for astETH to function.
    ///      In the case where it does not hold, the contract is only able to facilitate
    ///      emergency withdrawals, where users can receive their share of stETH in exchange for astETH.
    function _invariantCheck() internal view {
        uint256 stETHBalance = stETH.balanceOf(address(this));
        require(stETHBalance >= totalSupply(), "Invariant check failed");
    }
}
