import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

contract astETH is ERC20 {
    using SafeERC20 for IERC20;

    address public immutable stETH;
    address public immutable feeRecipient;

    constructor(
        address _stETH,
        address _feeRecipient
    ) ERC20("Alongside stETH", "astETH") {
        stETH = _stETH;
        feeRecipient = _feeRecipient;
    }

    function deposit(uint256 amount) external {
        IERC20(stETH).safeTransferFrom(msg.sender, address(this), amount);
        _mint(msg.sender, amount);
    }

    function withdraw(uint256 amount) external {
        _burn(msg.sender, amount);
        IERC20(stETH).safeTransfer(msg.sender, amount);
    }

    function collectFee() external {
        uint256 feeToCollect = IERC20(stETH).balanceOf(address(this)) -
            totalSupply();
        IERC20(stETH).safeTransfer(feeRecipient, feeToCollect);
    }
}
