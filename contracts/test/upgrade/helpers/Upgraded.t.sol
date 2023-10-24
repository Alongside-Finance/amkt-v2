pragma solidity =0.8.18;

import {UpgradePreparationTest} from "test/upgrade/helpers/UpgradePreparation.t.sol";

contract UpgradedTest is UpgradePreparationTest {
    bytes inputBatchExecutionData = hex"deadbabe";
    bool triggerMigrationWarning_executeUpgradeBundle;
    string forkOverrideUrl = "none";

    function setUp() public override {
        if (
            keccak256(abi.encodePacked(forkOverrideUrl)) ==
            keccak256(abi.encodePacked("none"))
        ) {
            super.setUp();
            executeUpgradeBundle();
        } else {
            vm.createSelectFork(forkOverrideUrl);
            super.setDeployedContracts();
        }
    }

    // WARN:
    // As a final check, the tests should be run with the actual calldata that signers will be using,
    // This serves as a redundant check that the calldata being signed is correct.
    function executeUpgradeBundle() public {
        triggerMigrationWarning_executeUpgradeBundle = true; // Flip when inputBatchExecution is used
        warpForward(1 hours); // there will be some time after we craft the batch, and we execute it
        executeBatchData(batchExecutionData); // use inputBatchExecutionData instead, when it is ready
    }
}
