// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Ownable2Step} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";

interface IStETH is IERC20 {
    function submit(address _referral) external payable returns (uint256);
}

interface IAstETH {
    event FeeRecipientSet(address feeRecipient);
    event FeeCollected(address feeRecipient, uint256 feeCollected);
}

contract AstETH is IAstETH, ERC20, Ownable2Step, ReentrancyGuard {
    using SafeERC20 for IStETH;

    IStETH public immutable stETH;
    address public feeRecipient;

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
    /// @dev Always mints 1 astETH for depositing 1 stETH
    function deposit(
        uint256 amountToMint
    ) external nonReentrant returns (uint256) {
        stETH.safeTransferFrom(msg.sender, address(this), amountToMint);
        _mint(msg.sender, amountToMint);
        return amountToMint;
    }

    /// @notice Wraps ETH to stETH before depositing stETH to receive astETH
    /// @dev Always mints 1 astETH for depositing 1 stETH
    function wrapAndDeposit() external payable nonReentrant returns (uint256) {
        uint256 balanceBefore = stETH.balanceOf(address(this));
        stETH.submit{value: msg.value}(address(0));
        uint256 balanceAfter = stETH.balanceOf(address(this));
        uint256 amountToMint = balanceAfter - balanceBefore;
        _mint(msg.sender, amountToMint);
        return amountToMint;
    }

    /// @notice Burn astETH to receive stETH
    /// @param  amountToBurn astETH amount to burn
    /// @dev Withdraws 1 stETH for burning 1 astETH
    /// @dev To protect against stETH rebasing down, amountToWithdraw is prorated if total supply exceeds stETH balance
    function withdraw(
        uint256 amountToBurn
    ) external nonReentrant returns (uint256) {
        uint totalStETH = stETH.balanceOf(address(this));
        uint amountToWithdraw = amountToBurn;
        if (totalStETH < totalSupply()) {
            amountToWithdraw = (amountToBurn * totalStETH) / totalSupply();
        }
        _burn(msg.sender, amountToBurn);
        stETH.safeTransfer(msg.sender, amountToWithdraw);
        return amountToWithdraw;
    }

    ///////////////////////// ADMIN /////////////////////////

    /// @notice Transfers excess stETH in the contract to the fee recipient
    function collectFee() external nonReentrant {
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
