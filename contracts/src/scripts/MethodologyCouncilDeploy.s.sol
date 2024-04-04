// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

import {Script} from "forge-std/Script.sol";
import "forge-std/Test.sol";

import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";

import {ActiveBounty} from "src/invoke/ActiveBounty.sol";
import {InvokeableBounty} from "src/invoke/Bounty.sol";

import {MULTISIG, VAULT} from "./Config.sol";

uint256 constant CANCELLATION_PERIOD = 3 days;

address constant PROPOSER_1 = 0xb9f16fb576af6065a2c775c24bdFAed5D468F1aD;
address constant PROPOSER_2 = 0xb9f16fb576af6065a2c775c24bdFAed5D468F1aD;
address constant PROPOSER_3 = 0xb9f16fb576af6065a2c775c24bdFAed5D468F1aD;
address constant PROPOSER_4 = 0xb9f16fb576af6065a2c775c24bdFAed5D468F1aD;

contract MethodologyCouncilDeployScript is Script {
    function run() public virtual returns (TimelockController, ActiveBounty, InvokeableBounty) {
        vm.startBroadcast(msg.sender);

        address[] memory emptyArr;

        TimelockController controller = new TimelockController(CANCELLATION_PERIOD, emptyArr, emptyArr, msg.sender);

        controller.grantRole(controller.PROPOSER_ROLE(), PROPOSER_1);
        controller.grantRole(controller.PROPOSER_ROLE(), PROPOSER_2);
        controller.grantRole(controller.PROPOSER_ROLE(), PROPOSER_3);
        controller.grantRole(controller.PROPOSER_ROLE(), PROPOSER_4);

        controller.grantRole(controller.CANCELLER_ROLE(), MULTISIG);
        controller.grantRole(controller.EXECUTOR_ROLE(), MULTISIG);

        controller.revokeRole(controller.TIMELOCK_ADMIN_ROLE(), msg.sender);

        ActiveBounty activeBounty = new ActiveBounty(address(controller));

        InvokeableBounty invokeableBounty = new InvokeableBounty({_vault: VAULT, _activeBounty: address(activeBounty)});

        vm.stopBroadcast();

        return (controller, activeBounty, invokeableBounty);
    }
}
