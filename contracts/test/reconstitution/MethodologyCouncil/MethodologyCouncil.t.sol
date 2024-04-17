// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

import {Test} from "forge-std/Test.sol";

import {
    MethodologyCouncilDeployScript, CANCELLATION_PERIOD, PROPOSER_1
} from "src/scripts/MethodologyCouncilDeploy.s.sol";
import {FulfillerDeployScript} from "src/scripts/FulfillerDeploy.s.sol";

import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";

import {Vault} from "src/Vault.sol";
import {IndexToken} from "src/IndexToken.sol";
import {ActiveBounty} from "src/invoke/ActiveBounty.sol";
import {InvokeableBounty} from "src/invoke/Bounty.sol";

import {AMKT_PROXY} from "src/scripts/Config.sol";

import "test/reconstitution/Reconstitution.t.sol";

contract MethodologyCouncilReconstitutionTest is ReconstitutionTest {
    TimelockController public methodologyCouncil;
    ActiveBounty public activeBounty;
    InvokeableBounty public invokeableBounty;

    function setUp() public override {
        fork();
        deployMethodologyCouncil();
        enableSimulation();

        FulfillerDeployScript fulfillerDeploy = new FulfillerDeployScript();
        fulfiller = fulfillerDeploy.run(address(invokeableBounty));

        postAndFulfillBounty();
    }

    function deployMethodologyCouncil() internal {
        MethodologyCouncilDeployScript deployScript = new MethodologyCouncilDeployScript();
        (methodologyCouncil, activeBounty, invokeableBounty) = deployScript.run();

        Vault vault = Vault(VAULT);

        vm.prank(vault.owner());
        vault.setRebalancer(address(invokeableBounty));
    }

    function postAndFulfillBounty() internal override {
        GnosisTransaction[] memory batch = new GnosisTransaction[](1);

        salt = keccak256(abi.encodePacked(IndexToken(AMKT_PROXY).totalSupply()));

        Bounty memory _bountyToSet =
            Bounty({infos: tokens(), fulfiller: address(fulfiller), salt: salt, deadline: block.timestamp + 30 days});
        bountyHash = invokeableBounty.hashBounty(_bountyToSet);

        vm.prank(PROPOSER_1);
        methodologyCouncil.schedule(
            address(activeBounty),
            0,
            abi.encodeWithSignature("setHash(bytes32)", bountyHash),
            bytes32(0),
            keccak256(abi.encodePacked("New Proposal for AMKT Recomposition!")),
            CANCELLATION_PERIOD
        );

        skip(CANCELLATION_PERIOD + 1);

        vm.prank(MULTISIG);
        methodologyCouncil.execute(
            address(activeBounty),
            0,
            abi.encodeWithSignature("setHash(bytes32)", bountyHash),
            bytes32(0),
            keccak256(abi.encodePacked("New Proposal for AMKT Recomposition!"))
        );

        triggerReconstitutionWarning_postBounty = false;
        bytes memory batchExecutionData = getBatchExecutionData(batch);
        executeBatchData(batchExecutionData);

        triggerReconstitutionWarning_fulfillBounty = false;

        fulfillerSafeTest = new FulfillerSafeTest(address(fulfiller));
        fulfillmentExecutionData = fulfillerSafeTest.runFulfillmentBatch(_bountyToSet);
    }
}
