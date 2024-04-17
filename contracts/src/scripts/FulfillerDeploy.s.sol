// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

import {Script} from "forge-std/Script.sol";
import {Fulfiller, INVOKEABLE_BOUNTY} from "periphery/Fulfiller.sol";

contract FulfillerDeployScript is Script {
    function run() public virtual returns (Fulfiller) {
        vm.startBroadcast(msg.sender);

        Fulfiller fulfiller = new Fulfiller(0x5ae65506979C00D70A13E7cE9eBf984d31660e5c, INVOKEABLE_BOUNTY);

        vm.stopBroadcast();

        return fulfiller;
    }

    function run(address invokeableBounty) public virtual returns (Fulfiller) {
        vm.startBroadcast(msg.sender);

        Fulfiller fulfiller = new Fulfiller(0x5ae65506979C00D70A13E7cE9eBf984d31660e5c, invokeableBounty);

        vm.stopBroadcast();

        return fulfiller;
    }
}
