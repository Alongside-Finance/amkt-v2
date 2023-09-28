pragma solidity =0.8.15;

import {UpgradeTest} from "./helpers/Upgrade.t.sol";
import {Dealer} from "test/Dealer.t.sol";
import {TokenInfo} from "src/Common.sol";
import {InitialBountyHelper, VOTE_DELAY, VOTE_PERIOD, CANCELLATION_PERIOD} from "src/scripts/Config.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {console} from "forge-std/console.sol";
import {IGovernor} from "@openzeppelin/contracts/governance/IGovernor.sol";

contract UpgradedFunctionalityTest is UpgradeTest {
    address largeAmktHolder =
        address(0x804B68f60765F4559b7096B158C912eD33aa0c26);

    struct Proposal {
        address[] targets;
        uint256[] values;
        bytes[] calldatas;
        string description;
        bytes32 descriptionHash;
    }

    function createSingleItemProposal(
        address target,
        uint256 value,
        bytes memory data,
        string memory description
    ) public returns (Proposal memory) {
        Proposal memory proposal = Proposal(
            new address[](1),
            new uint256[](1),
            new bytes[](1),
            description,
            keccak256(bytes(description))
        );
        proposal.targets[0] = target;
        proposal.values[0] = value;
        proposal.calldatas[0] = data;
        return proposal;
    }

    function testSetVotingDelay(
        uint256 delay,
        uint256 proposalDelayAmount,
        uint256 voteDelayAmount,
        uint256 queueDelayAmount,
        uint256 executeDelayAmount
    ) public {
        uint256 maxDelay = 365 days;
        proposalDelayAmount = bound(proposalDelayAmount, 0, maxDelay);
        voteDelayAmount = bound(voteDelayAmount, 0, maxDelay);
        queueDelayAmount = bound(queueDelayAmount, 0, maxDelay);
        executeDelayAmount = bound(executeDelayAmount, 0, maxDelay);
        vm.assume(proposalDelayAmount > 1 * AVG_BLOCK_TIME); // at least one block should have passed since delegating
        vm.assume(
            voteDelayAmount > (VOTE_DELAY + 1) * AVG_BLOCK_TIME &&
                voteDelayAmount < (VOTE_PERIOD + VOTE_DELAY) * AVG_BLOCK_TIME
        );
        vm.assume(queueDelayAmount > (VOTE_PERIOD + 1) * AVG_BLOCK_TIME);
        vm.assume(executeDelayAmount > CANCELLATION_PERIOD);
        // PROPOSE SETTING VOTING DELAY

        Proposal memory setVoteDelayProposal = createSingleItemProposal(
            address(governor),
            0,
            abi.encodeWithSignature("setVotingDelay(uint256)", delay),
            "Propose voting delay"
        );

        vm.startPrank(largeAmktHolder);
        // DELEGATE VOTES TO SELF
        AMKT.delegate(largeAmktHolder);
        warpForward(proposalDelayAmount);

        // PROPOSE
        uint256 proposalId = governor.propose(
            setVoteDelayProposal.targets,
            setVoteDelayProposal.values,
            setVoteDelayProposal.calldatas,
            setVoteDelayProposal.description
        );

        // PROPOSE EXECUTING DELAY PROPOSAL

        Proposal memory executeSetVoteDelayProposal = createSingleItemProposal(
            address(governor),
            0,
            abi.encodeWithSignature(
                "execute(address[],uint256[],bytes[],bytes32)",
                setVoteDelayProposal.targets,
                setVoteDelayProposal.values,
                setVoteDelayProposal.calldatas,
                setVoteDelayProposal.descriptionHash
            ),
            "Execute voting delay"
        );

        uint256 newProposalId = governor.propose(
            executeSetVoteDelayProposal.targets,
            executeSetVoteDelayProposal.values,
            executeSetVoteDelayProposal.calldatas,
            executeSetVoteDelayProposal.description
        );
        uint256 proposalTimestamp = block.timestamp;
        uint256 proposalBlock = block.number;
        vm.expectRevert("Governor: vote not currently active"); // voting before 1 day of delay should fail
        governor.castVote(newProposalId, 1);
        warpForward(voteDelayAmount);
        assertGe(block.number, governor.proposalSnapshot(proposalId));
        assertEq(
            uint256(governor.state(newProposalId)),
            uint256(IGovernor.ProposalState.Active)
        );
        governor.castVote(newProposalId, 1);
        governor.castVote(proposalId, 1);

        vm.expectRevert(); // queueing before 4 days of voting should fail
        governor.queue(
            executeSetVoteDelayProposal.targets,
            executeSetVoteDelayProposal.values,
            executeSetVoteDelayProposal.calldatas,
            executeSetVoteDelayProposal.descriptionHash
        );
        resetTimeAndBlock(proposalTimestamp, proposalBlock);
        warpForward((VOTE_DELAY + VOTE_PERIOD + 1) * AVG_BLOCK_TIME);
        warpForward(queueDelayAmount);
        governor.queue(
            executeSetVoteDelayProposal.targets,
            executeSetVoteDelayProposal.values,
            executeSetVoteDelayProposal.calldatas,
            executeSetVoteDelayProposal.descriptionHash
        );
        governor.queue(
            setVoteDelayProposal.targets,
            setVoteDelayProposal.values,
            setVoteDelayProposal.calldatas,
            setVoteDelayProposal.descriptionHash
        );

        // EXECUTE AFTER 1 DAY + 4 DAYS + 4 DAYS
        bytes32 operationId = timelockController.hashOperationBatch(
            executeSetVoteDelayProposal.targets,
            executeSetVoteDelayProposal.values,
            executeSetVoteDelayProposal.calldatas,
            0,
            executeSetVoteDelayProposal.descriptionHash
        );
        warpForward(executeDelayAmount);
        assertGe(block.timestamp, timelockController.getTimestamp(operationId));
        assertEq(timelockController.isOperation(operationId), true);
        assertEq(timelockController.isOperationReady(operationId), true);
        timelockController.executeBatch(
            executeSetVoteDelayProposal.targets,
            executeSetVoteDelayProposal.values,
            executeSetVoteDelayProposal.calldatas,
            0,
            executeSetVoteDelayProposal.descriptionHash
        );
        assertEq(governor.votingDelay(), delay);

        vm.stopPrank();
    }
}
