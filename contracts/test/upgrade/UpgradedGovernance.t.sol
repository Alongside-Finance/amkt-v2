pragma solidity =0.8.18;

import {UpgradedTest} from "test/upgrade/helpers/Upgraded.t.sol";
import {Dealer} from "test/utils/Dealer.t.sol";
import {TokenInfo} from "src/Common.sol";
import {InitialBountyHelper, VOTE_DELAY, VOTE_PERIOD, CANCELLATION_PERIOD, MULTISIG} from "src/scripts/Config.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {console} from "forge-std/console.sol";
import {IGovernor} from "@openzeppelin/contracts/governance/IGovernor.sol";
import {InvokeableBounty, IActiveBounty} from "src/invoke/Bounty.sol";

contract UpgradedGovernanceTest is UpgradedTest {
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
    ) public pure returns (Proposal memory) {
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

    function testRebalance() public {
        Proposal memory proposal = createSingleItemProposal(
            address(InvokeableBounty(vault.rebalancer()).activeBounty()),
            0,
            abi.encodeWithSignature("setHash(bytes32)", keccak256("hash")),
            "Test set hash"
        );
        makeProposalPass(proposal);
        assertEq(
            IActiveBounty(InvokeableBounty(vault.rebalancer()).activeBounty())
                .activeBounty(),
            keccak256("hash")
        );
    }

    function testAcceptVaultOwnership() public {
        Proposal memory proposal = createSingleItemProposal(
            address(vault),
            0,
            abi.encodeWithSignature("acceptOwnership()"),
            "Accept vault ownership"
        );
        makeProposalPass(proposal);
        assertEq(vault.owner(), address(timelockController));
    }

    function testCancelProposal() public {
        Proposal memory proposal = createSingleItemProposal(
            address(vault),
            0,
            abi.encodeWithSignature("acceptOwnership()"),
            "Accept vault ownership"
        );
        makeProposalCancel(proposal);
        assertNotEq(vault.owner(), address(timelockController));
    }

    function testTransfers() public {
        uint256 balance = AMKT.balanceOf(largeAmktHolder);
        vm.prank(largeAmktHolder);
        AMKT.transfer(address(1), balance / 2);
        assertEq(AMKT.getVotes(largeAmktHolder), 0);
        assertEq(AMKT.getVotes(address(1)), 0);
        vm.prank(largeAmktHolder);
        AMKT.delegate(address(1));
        assertEq(AMKT.getVotes(address(1)), balance / 2);
        vm.prank(largeAmktHolder);
        AMKT.transfer(address(2), 1e18);

        assertEq(AMKT.getVotes(largeAmktHolder), 0);
        assertEq(AMKT.getVotes(address(1)), balance / 2 - 1e18);
        assertEq(AMKT.getVotes(address(2)), 0);

        vm.prank(address(1));
        AMKT.delegate(largeAmktHolder);

        assertEq(AMKT.getVotes(largeAmktHolder), balance / 2);
        assertEq(AMKT.getVotes(address(1)), balance / 2 - 1e18);

        vm.prank(address(2));
        AMKT.delegate(address(2));

        assertEq(AMKT.getVotes(address(2)), 1e18);
    }

    /// forge-config: default.fuzz.runs = 20
    function testDelegateVoting(address user) public {
        vm.assume(user != address(0));
        vm.prank(largeAmktHolder);
        AMKT.delegate(user); // intermediate hop to user 1
        _warpForward(1 * AVG_BLOCK_TIME);
        assertEq(AMKT.getVotes(user), AMKT.balanceOf(largeAmktHolder));
        assertEq(
            AMKT.getPastVotes(user, block.number - 1),
            AMKT.balanceOf(largeAmktHolder)
        );
        Proposal memory proposal = createSingleItemProposal(
            address(vault),
            0,
            abi.encodeWithSignature("acceptOwnership()"),
            "Accept vault ownership"
        );
        vm.startPrank(user);
        uint256 proposalId = governor.propose(
            proposal.targets,
            proposal.values,
            proposal.calldatas,
            proposal.description
        );
        _warpForward((VOTE_DELAY + 1) * AVG_BLOCK_TIME);
        governor.castVote(proposalId, 1);
        vm.stopPrank();
    }

    function testTimelockSelfAdministration() public {
        bytes32[] memory roles = new bytes32[](4);
        roles[0] = timelockController.TIMELOCK_ADMIN_ROLE();
        roles[1] = timelockController.PROPOSER_ROLE();
        roles[2] = timelockController.EXECUTOR_ROLE();
        roles[3] = timelockController.CANCELLER_ROLE();

        vm.startPrank(MULTISIG);
        for (uint256 i = 0; i < roles.length; i++) {
            timelockController.schedule(
                address(timelockController),
                0,
                abi.encodeWithSignature(
                    "grantRole(bytes32,address)",
                    roles[i],
                    address(2)
                ),
                0,
                keccak256("hash"),
                timelockController.getMinDelay()
            );
        }
        vm.stopPrank();
    }

    /// forge-config: default.fuzz.runs = 20
    function testOnlyProposerCanProposeTimelock(address stranger) public {
        vm.assume(
            stranger != address(DEFAULT_TEST_CONTRACT) &&
                stranger != MULTISIG &&
                stranger != address(governor)
        );
        uint256 delay = timelockController.getMinDelay();
        vm.startPrank(stranger);
        vm.expectRevert();
        timelockController.schedule(
            address(vault),
            0,
            abi.encodeWithSignature("acceptOwnership()"),
            0,
            keccak256("hash"),
            delay
        );
        vm.stopPrank();
    }

    function testMultisigTimelockProposal() public {
        vm.startPrank(MULTISIG);
        timelockController.schedule(
            address(vault),
            0,
            abi.encodeWithSignature("acceptOwnership()"),
            0,
            keccak256("hash"),
            timelockController.getMinDelay()
        );
        vm.stopPrank();
    }

    function makeProposalPass(Proposal memory proposal) public {
        vm.startPrank(largeAmktHolder);
        AMKT.delegate(largeAmktHolder);
        vm.expectRevert(); // too early
        governor.propose(
            proposal.targets,
            proposal.values,
            proposal.calldatas,
            proposal.description
        );
        _warpForward(1 * AVG_BLOCK_TIME);
        uint256 proposalId = governor.propose(
            proposal.targets,
            proposal.values,
            proposal.calldatas,
            proposal.description
        );
        vm.expectRevert(); // too early
        governor.castVote(proposalId, 1);
        _warpForward((VOTE_DELAY + 1) * AVG_BLOCK_TIME);
        governor.castVote(proposalId, 1);
        vm.expectRevert(); // too early
        governor.queue(
            proposal.targets,
            proposal.values,
            proposal.calldatas,
            proposal.descriptionHash
        );
        _warpForward((VOTE_PERIOD + 1) * AVG_BLOCK_TIME);
        governor.queue(
            proposal.targets,
            proposal.values,
            proposal.calldatas,
            proposal.descriptionHash
        );
        vm.stopPrank();

        vm.startPrank(MULTISIG);
        vm.expectRevert();
        timelockController.executeBatch(
            proposal.targets,
            proposal.values,
            proposal.calldatas,
            0,
            proposal.descriptionHash
        );
        _warpForward(CANCELLATION_PERIOD);
        timelockController.executeBatch(
            proposal.targets,
            proposal.values,
            proposal.calldatas,
            0,
            proposal.descriptionHash
        );

        bytes32 operationId = timelockController.hashOperationBatch(
            proposal.targets,
            proposal.values,
            proposal.calldatas,
            0,
            proposal.descriptionHash
        );
        assertEq(timelockController.isOperation(operationId), true);
        assertEq(timelockController.isOperationDone(operationId), true);
        vm.stopPrank();
    }

    function makeProposalCancel(Proposal memory proposal) public {
        vm.startPrank(largeAmktHolder);
        AMKT.delegate(largeAmktHolder);
        _warpForward(1 * AVG_BLOCK_TIME);
        uint256 proposalId = governor.propose(
            proposal.targets,
            proposal.values,
            proposal.calldatas,
            proposal.description
        );
        _warpForward((VOTE_DELAY + 1) * AVG_BLOCK_TIME);
        governor.castVote(proposalId, 1);
        _warpForward((VOTE_PERIOD + 1) * AVG_BLOCK_TIME);
        governor.queue(
            proposal.targets,
            proposal.values,
            proposal.calldatas,
            proposal.descriptionHash
        );
        vm.stopPrank();

        _warpForward(CANCELLATION_PERIOD - 1);

        bytes32 operationId = timelockController.hashOperationBatch(
            proposal.targets,
            proposal.values,
            proposal.calldatas,
            0,
            proposal.descriptionHash
        );
        vm.prank(MULTISIG);
        timelockController.cancel(operationId);
        assertEq(timelockController.isOperation(operationId), false);
    }

    function testSetVotingDelay(
        uint256 delay,
        uint256 proposalDelayAmount,
        uint256 voteDelayAmount,
        uint256 queueDelayAmount,
        uint256 executeDelayAmount
    ) public {
        uint256 maxDelay = 365 days;
        proposalDelayAmount = bound(
            proposalDelayAmount,
            1 * AVG_BLOCK_TIME,
            maxDelay
        );
        voteDelayAmount = bound(
            voteDelayAmount,
            (VOTE_DELAY + 1) * AVG_BLOCK_TIME,
            (VOTE_PERIOD + VOTE_DELAY) * AVG_BLOCK_TIME
        );
        queueDelayAmount = bound(
            queueDelayAmount,
            (VOTE_PERIOD + 1) * AVG_BLOCK_TIME,
            maxDelay
        );
        executeDelayAmount = bound(
            executeDelayAmount,
            CANCELLATION_PERIOD,
            maxDelay
        );
        vm.assume(proposalDelayAmount > 1 * AVG_BLOCK_TIME); // at least one block should have passed since delegating
        vm.assume(voteDelayAmount > (VOTE_DELAY + 1) * AVG_BLOCK_TIME);
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
        _warpForward(proposalDelayAmount);

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
        _warpForward(voteDelayAmount);
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
        _resetTimeAndBlock(proposalTimestamp, proposalBlock);
        _warpForward((VOTE_DELAY + VOTE_PERIOD + 1) * AVG_BLOCK_TIME);
        _warpForward(queueDelayAmount);
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
        vm.stopPrank();

        vm.startPrank(MULTISIG);
        // sandwich the execution of the proposal between granting and revoking the executor role to the governor.
        // this is the only way governor can call `execute` on itself.
        timelockController.schedule(
            address(timelockController),
            0,
            abi.encodeWithSignature(
                "grantRole(bytes32,address)",
                timelockController.EXECUTOR_ROLE(),
                address(governor)
            ),
            0,
            keccak256("hash"),
            timelockController.getMinDelay()
        );
        timelockController.schedule(
            address(timelockController),
            0,
            abi.encodeWithSignature(
                "revokeRole(bytes32,address)",
                timelockController.EXECUTOR_ROLE(),
                address(governor)
            ),
            0,
            keccak256("hash"),
            timelockController.getMinDelay()
        );

        // EXECUTE AFTER 1 DAY + 4 DAYS + 4 DAYS
        bytes32 operationId = timelockController.hashOperationBatch(
            executeSetVoteDelayProposal.targets,
            executeSetVoteDelayProposal.values,
            executeSetVoteDelayProposal.calldatas,
            0,
            executeSetVoteDelayProposal.descriptionHash
        );
        _warpForward(executeDelayAmount);
        assertGe(block.timestamp, timelockController.getTimestamp(operationId));
        assertEq(timelockController.isOperation(operationId), true);
        assertEq(timelockController.isOperationReady(operationId), true);
        // TODO: Execute all in batch
        timelockController.execute(
            address(timelockController),
            0,
            abi.encodeWithSignature(
                "grantRole(bytes32,address)",
                timelockController.EXECUTOR_ROLE(),
                address(governor)
            ),
            0,
            keccak256("hash")
        );
        timelockController.executeBatch(
            executeSetVoteDelayProposal.targets,
            executeSetVoteDelayProposal.values,
            executeSetVoteDelayProposal.calldatas,
            0,
            executeSetVoteDelayProposal.descriptionHash
        );
        timelockController.execute(
            address(timelockController),
            0,
            abi.encodeWithSignature(
                "revokeRole(bytes32,address)",
                timelockController.EXECUTOR_ROLE(),
                address(governor)
            ),
            0,
            keccak256("hash")
        );
        assertEq(governor.votingDelay(), delay);

        vm.stopPrank();
    }
}
