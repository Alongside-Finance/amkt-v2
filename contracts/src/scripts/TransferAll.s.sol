pragma solidity =0.8.18;

import {Script} from "forge-std/Script.sol";
import {TransferAll} from "periphery/TransferAll.sol";

contract TransferAllScript is Script {
    function run() public {
        vm.startBroadcast();
        new TransferAll();
        vm.stopBroadcast();
    }
}
