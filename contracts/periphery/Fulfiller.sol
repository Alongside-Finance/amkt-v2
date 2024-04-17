// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

import {Ownable2Step} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {Bounty} from "src/interfaces/IInvokeableBounty.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IInvokeableBounty} from "src/interfaces/IInvokeableBounty.sol";

address constant WSTETH = address(0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0);
address constant STETH = address(0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84);
address constant ASTETH = address(0x27C2B9fd547EAd2c05C305BeE2399A55811257c2);
address constant INVOKEABLE_BOUNTY = address(0xE13Ee59C41c67696754277cDC73710f6D65Ef2Ac);

interface wstETH {
    function unwrap(uint256 wstETHAmount) external returns (uint256 stETHAmount);
}

interface astETH {
    function deposit(uint256 stETHAmount) external returns (uint256 astETHAmount);
}

// To be used for Jan 2024 reconstitution by a fulfiller
contract Fulfiller is Ownable2Step {
    address public invokeableBounty;

    constructor(address owner, address _invokeableBounty) {
        _transferOwnership(owner);
        require(_invokeableBounty != address(0), "Zero Address");
        invokeableBounty = _invokeableBounty;
    }

    // In case the owner wants to transfer any tokens out
    function withdrawERC20(address token, uint256 amount) external onlyOwner {
        IERC20(token).transfer(owner(), amount);
    }

    // Combine token approvals and fulfillBounty into one function
    function fulfillBounty(Bounty calldata bounty, bool callback) external onlyOwner {
        for (uint256 i; i < bounty.infos.length; i++) {
            IERC20(bounty.infos[i].token).approve(invokeableBounty, type(uint256).max);
        }
        IInvokeableBounty(invokeableBounty).fulfillBounty(bounty, callback);
    }

    // Upon receiving the callback, unwrap wstETH into stETH and deposit stETH into astETH
    function rebalanceCallback(TokenInfo[] calldata required, TokenInfo[] calldata received) external {
        require(msg.sender == invokeableBounty, "invalid sender");
        for (uint256 i; i < received.length; i++) {
            if (received[i].token == WSTETH) {
                uint256 wstETHAmount = received[i].units;
                uint256 stETHAmount = wstETH(WSTETH).unwrap(wstETHAmount);
                IERC20(STETH).approve(ASTETH, stETHAmount);
                astETH(ASTETH).deposit(stETHAmount);
            }
        }
    }
}
