pragma solidity =0.8.15;

import {UpgradeTest} from "./helpers/Upgrade.t.sol";
import {Dealer} from "test/Dealer.t.sol";
import {TokenInfo} from "src/Common.sol";
import {InitialBountyHelper} from "src/scripts/Config.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {console} from "forge-std/console.sol";

contract UpgradedFunctionalityTest is UpgradeTest {
    address largeAmktHolder =
        address(0x804B68f60765F4559b7096B158C912eD33aa0c26);

    function testSetVotingDelay() public {
        uint256 AVG_BLOCK_TIME = 12;
        // PROPOSE SETTING VOTING DELAY
        bytes memory setVotingDelayData = abi.encodeWithSignature(
            "setVotingDelay(uint256)",
            1000
        );
        address[] memory targets = new address[](1);
        targets[0] = address(governor);
        uint256[] memory values = new uint256[](1);
        values[0] = 0;
        bytes[] memory calldatas = new bytes[](1);
        calldatas[0] = setVotingDelayData;
        string memory description = "Propose voting delay";

        vm.startPrank(largeAmktHolder);
        // DELEGATE VOTES TO SELF
        AMKT.delegate(largeAmktHolder);
        vm.roll(block.number + 1);

        // PROPOSE
        uint256 proposalId = governor.propose(
            targets,
            values,
            calldatas,
            description
        );
        bytes32 descriptionHash = keccak256(bytes(description));

        // PROPOSE EXECUTING DELAY PROPOSAL
        bytes memory executeData = abi.encodeWithSignature(
            "execute(address[],uint256[],bytes[],bytes32)",
            targets,
            values,
            calldatas,
            descriptionHash
        );

        bytes[] memory newCalldatas = new bytes[](1);
        newCalldatas[0] = executeData;
        string memory newDescription = "Execute voting delay";
        bytes32 newDescriptionHash = keccak256(bytes(newDescription));
        uint256 newProposalId = governor.propose(
            targets,
            values,
            newCalldatas,
            newDescription
        );
        // VOTE AFTER 1 DAY
        vm.expectRevert(); // voting before 1 day of delay should fail
        governor.castVote(newProposalId, 1);
        vm.roll(block.number + 1 days / AVG_BLOCK_TIME + 1);
        governor.castVote(newProposalId, 1);
        governor.castVote(proposalId, 1);
        // QUEUE AFTER 1 DAY + 4 DAYS
        vm.expectRevert(); // queueing before 4 days of voting should fail
        governor.queue(targets, values, newCalldatas, newDescriptionHash);
        vm.roll(block.number + 4 days / AVG_BLOCK_TIME);
        governor.queue(targets, values, newCalldatas, newDescriptionHash);
        governor.queue(targets, values, calldatas, descriptionHash);

        // EXECUTE AFTER 1 DAY + 4 DAYS + 4 DAYS
        bytes32 operationId = timelockController.hashOperationBatch(
            targets,
            values,
            newCalldatas,
            0,
            newDescriptionHash
        );
        uint256 timestamp = timelockController.getTimestamp(operationId);
        vm.warp(timestamp);
        assertEq(timelockController.isOperation(operationId), true);
        assertEq(timelockController.isOperationReady(operationId), true);
        timelockController.executeBatch(
            targets,
            values,
            newCalldatas,
            0,
            newDescriptionHash
        );
        assertEq(governor.votingDelay(), 1000);

        vm.stopPrank();
    }
}
