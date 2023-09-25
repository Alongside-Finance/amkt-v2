pragma solidity =0.8.15;

import "forge-std/Test.sol";
import {MULTISIG} from "src/scripts/Config.sol";
import {console} from "forge-std/console.sol";
import {GnosisTest, GnosisTransaction} from "./Gnosis.sol";

contract UpgradeTest is GnosisTest {
    function testForkUpgrade() public {
        vm.createSelectFork(vm.envString("MAINNET_RPC"));
        vm.roll(18042389 - 1);
        enableSimulation();
        executeBatch(createTestBatch());
    }

    function createTestBatch() public returns (GnosisTransaction[] memory) {
        GnosisTransaction[] memory batch = new GnosisTransaction[](2);
        address guyToApprove = address(0xdeadbabe);
        address weth = 0xF17A3fE536F8F7847F1385ec1bC967b2Ca9caE8D;

        // Sample transaction 1
        bytes4 approveFunctionSignature = bytes4(
            keccak256("approve(address,uint256)")
        );
        uint256 wad1 = 100;
        bytes memory approveData1 = abi.encodeWithSelector(
            approveFunctionSignature,
            guyToApprove,
            wad1
        );
        batch[0] = GnosisTransaction({to: weth, value: 0, data: approveData1});

        // Sample transaction 2
        uint256 wad2 = 200;
        bytes memory approveData2 = abi.encodeWithSelector(
            approveFunctionSignature,
            guyToApprove,
            wad2
        );
        batch[1] = GnosisTransaction({to: weth, value: 0, data: approveData2});

        return batch;
    }
}
