// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Ownable2Step} from "@openzeppelin/contracts/access/Ownable2Step.sol";

interface IStETH is IERC20 {
    function submit(address _referral) external payable returns (uint256);
}

interface IAstETH {
    error Reentrant();
    event FeeRecipientSet(address feeRecipient);
    event FeeCollected(address feeRecipient, uint256 feeCollected);
}

contract AstETH is IAstETH, ERC20, Ownable2Step {
    using SafeERC20 for IStETH;

    IStETH public immutable stETH;
    address public feeRecipient;

    uint256 public reentrancyLock = 1;

    modifier reentrancyGuard() {
        if (reentrancyLock > 1) revert Reentrant();
        reentrancyLock = 2;
        _;
        reentrancyLock = 1;
    }

    constructor(
        IStETH _stETH,
        address _owner,
        address _feeRecipient
    ) ERC20("Alongside stETH", "astETH") {
        stETH = _stETH;
        feeRecipient = _feeRecipient;
        _transferOwnership(_owner);
    }

    ///////////////////////// ISSUANCE /////////////////////////

    /// @notice Deposit stETH to receive astETH
    /// @param  amountToMint stETH amount to deposit
    function deposit(
        uint256 amountToMint
    ) external reentrancyGuard returns (uint256) {
        stETH.safeTransferFrom(msg.sender, address(this), amountToMint);
        _mint(msg.sender, amountToMint);
        return amountToMint;
    }

    /// @notice Wraps ETH to stETH before depositing stETH to receive astETH
    function wrapAndDeposit()
        external
        payable
        reentrancyGuard
        returns (uint256)
    {
        uint256 balanceBefore = stETH.balanceOf(address(this));
        stETH.submit{value: msg.value}(address(0));
        uint256 balanceAfter = stETH.balanceOf(address(this));
        uint256 amountToMint = balanceAfter - balanceBefore;
        _mint(msg.sender, amountToMint);
        return amountToMint;
    }

    /// @notice Burn astETH to receive stETH
    /// @param  amountToBurn astETH amount to burn
    function withdraw(uint256 amountToBurn) external returns (uint256) {
        uint totalstETH = stETH.balanceOf(address(this));
        uint amountToWithdraw = amountToBurn;
        if (totalstETH < totalSupply()) {
            amountToWithdraw = (amountToBurn * totalstETH) / totalSupply();
        }
        _burn(msg.sender, amountToBurn);
        stETH.safeTransfer(msg.sender, amountToWithdraw);
        return amountToWithdraw;
    }

    ///////////////////////// ADMIN /////////////////////////

    /// @notice Transfers excess stETH in the contract to the fee recipient
    function collectFee() external {
        uint256 stETHBalance = stETH.balanceOf(address(this));
        uint256 feeToCollect = stETHBalance - totalSupply();
        stETH.safeTransfer(feeRecipient, feeToCollect);
        emit FeeCollected(feeRecipient, feeToCollect);
    }

    /// @param _feeRecipient address of new fee recipient
    function setFeeRecipient(address _feeRecipient) external onlyOwner {
        feeRecipient = _feeRecipient;
        emit FeeRecipientSet(feeRecipient);
    }
}
