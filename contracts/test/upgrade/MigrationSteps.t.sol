pragma solidity =0.8.18;

import {Test} from "forge-std/Test.sol";

// NOTE TO DEVELOPERS:
// This checklist test is used to ensure safety of the migration process.
// These tests serve as a guide to the migration process, step by step.
// Each step should be marked as complete switching `false` to `true` in `assertTrue`.
contract MigrationStepsTest is Test {
    function testStep0_deployContracts() public {
        // Deploy contracts using CoreDeploy.s.sol and set the contract addresses in:
        // `UpgradePreparation` and `test_MIGRATION_WARNING_deployedContracts`
        // Set `triggerMigrationWarning_setDeployedContracts` in `setDeployedContract` to `false` upon completion
        assertTrue(false);
    }

    function testStep1_setFinalTokenUnits() public {
        // By step 1, it is assumed that all the funds have been transferred to the multisig.
        // Update InitialBountyHelper's token's list to reflect the final bounty units.
        // Set `triggerMigrationWarning_finalTokens` in `Config` to `false` upon completion
        assertTrue(false);
    }

    function testStep2_stopMockingSafeBalances() public {
        // Remove `mockSafeBalances` from `UpgradePreparation`
        assertTrue(false);
    }

    function testStep3_removeForkBlock() public {
        // Remove `fork` from `UpgradePreparation`.
        // NOTE: Foundry may cache forked state. Use `forge clean` every time you want to run a clean fork.
        assertTrue(false);
    }

    function testStep4_updateCurrentPrice() public {
        // Update `MIGRATION_WARNING_getCurrentPrice` in `MigrationChecklist` to reflect the latest prices.
        // Set `triggerMigrationWarning_getCurrentPrice` in `MigrationChecklist` to `false` upon completion
        assertTrue(false);
    }

    function testStep5_updateNav() public {
        // Update `nav` in `MigrationChecklist` to reflect the latest nav.
        assertTrue(false);
    }

    function testStep6_removeWarpForward() public {
        // Remove `warpForward` in `UpgradePreparation`
        assertTrue(false);
    }

    function testStep7_fixTokenState() public {
        // `testTokenState` in `UpgradedState.t.sol` should be updated to reflect the final token state.
        assertTrue(false);
    }

    function testStep8_insertInputCalldata() public {
        // Insert the calldata for `inputBatchExecutionData` in `Upgraded.t.sol`
        // Set `triggerMigrationWarning_executeUpgradeBundle` in `Upgraded.t.sol` to `false` upon completion
        assertTrue(false);
    }

    function testStep9_proposeCalldata() public {
        // Propose calldata to multisig
        assertTrue(false);
    }

    function testStep10_tenderlyFork() public {
        // Test state of tenderly fork of the upgrade bundle
        // Set `forkOverrideUrl` in `Upgraded.t.sol` to the tenderly fork rpc url
        assertTrue(false);
    }

    function testStep11_executeCalldata() public {
        // Execute calldata via multisig
        assertTrue(false);
    }

    function testStep12_mainnet() public {
        // Test state of the contracts on mainnet after the upgrade
        // Set `forkOverrideUrl` in `Upgraded.t.sol` to the mainnet rpc url
        assertTrue(false);
    }
}
