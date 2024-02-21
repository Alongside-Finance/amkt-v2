pragma solidity =0.8.18;

import "forge-std/Test.sol";

import "src/IndexToken.sol";
import "src/Governor.sol";
import "src/Vault.sol";

import {FEE_RECEIPIENT, MULTISIG, AMKT_PROXY, GOVERNOR, TIMELOCK, VAULT} from "src/scripts/Config.sol";

contract Proposals is Test {
    AlongsideGovernor public governor;
    TimelockController public timelock;
    Vault public vault;
    IndexToken public amkt;

    function setUp() public {
        governor = AlongsideGovernor(GOVERNOR);
        timelock = TimelockController(TIMELOCK);
        amkt = IndexToken(AMKT_PROXY);
        vault = Vault(VAULT);
    }

    function test_Proposal_InflationRate0() public {
        vm.createSelectFork(vm.envString("MAINNET_RPC"), 19270124);

        address[] memory targets = new address[](1);
        uint256[] memory values = new uint256[](1);
        bytes[] memory calldatas = new bytes[](1);

        targets[0] = VAULT;
        values[0] = 0;
        calldatas[0] = abi.encodeWithSignature("setInflationRate(uint256)", 0);

        // Execute Proposal.
        vm.prank(MULTISIG);
        timelock.executeBatch(
            targets,
            values,
            calldatas,
            0,
            0xb8cc12a54947ab7c37a32bb425bd0aa6ecb8ac8add3d5911dcf70503fa31b47f
        );

        uint256 balBefore = amkt.balanceOf(FEE_RECEIPIENT);

        // Check that previously accrued inflation has been lost (intended)
        vm.prank(FEE_RECEIPIENT);
        vm.expectRevert(IVault.AMKTVaultFeeTooSmall.selector);
        vault.tryInflation();

        uint256 balAfter = amkt.balanceOf(FEE_RECEIPIENT);

        assertEq(balBefore, balAfter, "Inflation distributed.");

        skip(100 days);
        // Check that won't accrue fees in a future
        vm.prank(FEE_RECEIPIENT);
        vm.expectRevert(IVault.AMKTVaultFeeTooSmall.selector);
        vault.tryInflation();

        balAfter = amkt.balanceOf(FEE_RECEIPIENT);

        assertEq(balBefore, balAfter, "Inflation distributed.");

        // Check that inflation can be reactivated
        vm.prank(TIMELOCK);
        vault.setInflationRate(304132280);

        balAfter = amkt.balanceOf(FEE_RECEIPIENT);

        skip(10 days);

        vm.prank(FEE_RECEIPIENT);
        vault.tryInflation();
    }

    function test_Proposal_SecurityCouncil() public {
        vm.createSelectFork(vm.envString("MAINNET_RPC"), 19270124);

        uint256 proposalId = 39210095563482212864324380867477677335833373512568812460888567846573183331740;
        bytes32 operation = 0x7bfa0b4c7f95277938c7578255e1bf3e3ea14ba379380e20c7e1b4a7df44d51d;

        address[] memory council = new address[](3);

        council[0] = 0xf346100e892553DcEb41A927Fb668DA7B0b7C964;
        council[1] = 0x965B94c146c8D7A30242e83F4dD17a97f0aC3F4D;
        council[2] = 0x34b0711Aa7A71Eaba1316170DF6449c7e13fC434;

        require(
            governor.state(proposalId) == IGovernor.ProposalState.Queued,
            "Proposal is not queued."
        );

        // Build Operation
        address[] memory targets = new address[](4);
        uint256[] memory values = new uint256[](4);
        bytes[] memory calldatas = new bytes[](4);

        for (uint256 i; i < 4; ++i) {
            targets[i] = TIMELOCK;
            values[i] = 0;
            if (i < 3) {
                calldatas[i] = abi.encodeWithSignature(
                    "grantRole(bytes32,address)",
                    0xb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc1,
                    council[i]
                );
            } else {
                calldatas[i] = abi.encodeWithSignature(
                    "revokeRole(bytes32,address)",
                    0xb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc1,
                    MULTISIG
                );
            }
        }

        bytes32 proposalHash = timelock.hashOperationBatch(
            targets,
            values,
            calldatas,
            0,
            0x729f9cb7f5c30a1c71a0c2a6c8544834eb266b24bf3c4ccc51c1a44104f58220
        );

        assertEq(proposalHash, operation, "Operation not built properly.");

        // Execute Proposal.
        vm.prank(MULTISIG);
        timelock.executeBatch(
            targets,
            values,
            calldatas,
            0,
            0x729f9cb7f5c30a1c71a0c2a6c8544834eb266b24bf3c4ccc51c1a44104f58220
        );

        // Check that the roles have been properly added to the security council and revoked for the multisig.
        assertTrue(
            timelock.hasRole(
                0xb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc1,
                council[0]
            ),
            "Security Council 1 Doesn't have proposer role."
        );
        assertTrue(
            timelock.hasRole(
                0xb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc1,
                council[1]
            ),
            "Security Council 2 Doesn't have proposer role."
        );
        assertTrue(
            timelock.hasRole(
                0xb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc1,
                council[2]
            ),
            "Security Council 3 Doesn't have proposer role."
        );
        assertFalse(
            timelock.hasRole(
                0xb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc1,
                MULTISIG
            ),
            "Alongside multisig still has proposer role."
        );

        // Check that the security council cannot cancel proposals.
        for (uint i; i < 3; ++i) {
            vm.prank(council[i]);
            vm.expectRevert();
            timelock.cancel(
                0x77435c7fdb72fbfc9caab88d49d3ff37bb58aedc321bd0d7e3fac9b105a43dbf
            );
        }

        // Check that alongside multisig can still cancel proposals.
        uint256 snapshotPreCancel = vm.snapshot();
        vm.prank(MULTISIG);
        timelock.cancel(
            0x77435c7fdb72fbfc9caab88d49d3ff37bb58aedc321bd0d7e3fac9b105a43dbf
        );

        // Check that the security council cannot execute proposals.
        uint256 readyTimestamp = timelock.getTimestamp(
            0x77435c7fdb72fbfc9caab88d49d3ff37bb58aedc321bd0d7e3fac9b105a43dbf
        );

        vm.warp(readyTimestamp);

        address[] memory targetsInflation = new address[](1);
        uint256[] memory valuesInflation = new uint256[](1);
        bytes[] memory calldatasInflation = new bytes[](1);

        targetsInflation[0] = VAULT;
        valuesInflation[0] = 0;
        calldatasInflation[0] = abi.encodeWithSignature(
            "setInflationRate(uint256)",
            0
        );

        vm.revertTo(snapshotPreCancel);
        for (uint i; i < 3; ++i) {
            vm.prank(council[i]);
            vm.expectRevert();
            timelock.executeBatch(
                targetsInflation,
                valuesInflation,
                calldatasInflation,
                0,
                0xb8cc12a54947ab7c37a32bb425bd0aa6ecb8ac8add3d5911dcf70503fa31b47f
            );
        }

        // Check that alongside multisig can execute proposals.
        assertEq(
            timelock.hashOperationBatch(
                targetsInflation,
                valuesInflation,
                calldatasInflation,
                0,
                0xb8cc12a54947ab7c37a32bb425bd0aa6ecb8ac8add3d5911dcf70503fa31b47f
            ),
            0x77435c7fdb72fbfc9caab88d49d3ff37bb58aedc321bd0d7e3fac9b105a43dbf
        );

        vm.prank(MULTISIG);
        timelock.executeBatch(
            targetsInflation,
            valuesInflation,
            calldatasInflation,
            0,
            0xb8cc12a54947ab7c37a32bb425bd0aa6ecb8ac8add3d5911dcf70503fa31b47f
        );
    }
}
