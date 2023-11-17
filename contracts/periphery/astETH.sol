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
    function deposit(uint256 amount) external {
        stETH.safeTransferFrom(msg.sender, address(this), amount);
        _mint(msg.sender, amount);
    }

    /// @notice Burn astETH to receive stETH
    /// @param  sharesToBurn astETH amount to burn
    function withdraw(uint256 sharesToBurn) external {
        uint totalstETH = IERC20(stETH).balanceOf(address(this));
        uint amountToWithdraw = sharesToBurn;
        if (totalstETH < totalSupply()) {
            amountToWithdraw = (sharesToBurn * totalstETH) / totalSupply();
        }
        _burn(msg.sender, sharesToBurn);
        stETH.safeTransfer(msg.sender, amountToWithdraw);
    }

    /// @notice Transfers excess stETH in the contract to the fee recipient
    function collectFee() external {
        uint256 stETHBalance = stETH.balanceOf(address(this));
        uint256 feeToCollect = stETHBalance - totalSupply();
        stETH.safeTransfer(feeRecipient, feeToCollect);
    }

    ///////////////////////// OWNER /////////////////////////

    /// @param _feeRecipient address of new fee recipient
    function setFeeRecipient(address _feeRecipient) external onlyOwner {
        feeRecipient = _feeRecipient;
    }
}
