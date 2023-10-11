pragma solidity =0.8.18;

import {UpgradePreparationTest} from "test/upgrade/helpers/UpgradePreparation.t.sol";
import {console} from "forge-std/console.sol";

contract UpgradedTest is UpgradePreparationTest {
    function setUp() public override {
        super.setUp();
        MIGRATION_WARNING_executeUpgradeBundle();
    }

    // WARN:
    // As a final check, the tests should be run with the actual calldata that signers will be using,
    // This serves as a redundant check that the calldata being signed is correct.
    function MIGRATION_WARNING_executeUpgradeBundle() public {
        _warpForward(1 hours); // there will be some time after we craft the batch, and we execute it
        bool shouldPrintExecutionData = false;
        if (shouldPrintExecutionData) {
            console.logBytes(batchExecutionData);
        }
        executeBatchData(batchExecutionData);
    }
}
