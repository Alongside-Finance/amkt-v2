pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {AlongsideGovernor} from "src/Governor.sol";
import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";
import {ERC20VotesMock} from "@openzeppelin/contracts/mocks/ERC20VotesMock.sol";
import {CANCELLATION_PERIOD, VOTE_DELAY, VOTE_PERIOD, PROPOSAL_THRESHOLD, GOVERNOR_NUMERATOR} from "src/scripts/Config.sol";

contract GovernorTest is Test {
    ERC20VotesMock private token;
    TimelockController private timelock;
    AlongsideGovernor private governor;

    function setUp() public {
        token = new ERC20VotesMock("Index", "INDEX");
        timelock = new TimelockController(
            CANCELLATION_PERIOD,
            new address[](0),
            new address[](0),
            msg.sender
        ); // example settings

        governor = new AlongsideGovernor(token, timelock);

        // Mint tokens to simulate multiple voters.
        token.mint(address(this), 1000000 ether);
        token.approve(address(governor), 1000000 ether);
    }

    function testVotingDelay() public {
        uint256 expectedDelay = VOTE_DELAY;
        assertEq(governor.votingDelay(), expectedDelay);
    }

    function testVotingPeriod() public {
        uint256 expectedPeriod = VOTE_PERIOD;
        assertEq(governor.votingPeriod(), expectedPeriod);
    }

    function testProposalThreshold() public {
        uint256 expectedThreshold = PROPOSAL_THRESHOLD;
        assertEq(governor.proposalThreshold(), expectedThreshold);
    }

    function testQuorum() public {
        uint256 blockNumber = block.number; // Current block
        vm.roll(block.number + 1); // set block number higher
        uint256 expectedQuorum = GOVERNOR_NUMERATOR;
        uint256 governorDemoniator = 10000;
        assertEq(
            governor.quorum(blockNumber),
            (expectedQuorum * token.totalSupply()) / governorDemoniator
        );
    }
}
