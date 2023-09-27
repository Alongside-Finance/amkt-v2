pragma solidity =0.8.15;

import {UpgradeTest} from "./Upgrade.t.sol";
import {Dealer} from "test/Dealer.t.sol";
import {TokenInfo} from "src/Common.sol";
import {InitialBountyHelper} from "src/scripts/Config.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {console} from "forge-std/console.sol";

contract UpgradedFunctionality is UpgradeTest {
    address largeAmktHolder =
        address(0x804B68f60765F4559b7096B158C912eD33aa0c26);
    address oldMinter = address(0x0D44F856E1a7c70E35c54261c3f07DbFBDCA4857);

    function testTryInflation() public {
        vault.tryInflation();
        vm.prank(address(2));
        uint256 beforeSupply = AMKT.totalSupply();
        vault.tryInflation();
        assertEq(AMKT.totalSupply(), beforeSupply);

        vm.warp(block.timestamp + 1 days);
        vault.tryInflation();
        assertGe(AMKT.totalSupply(), beforeSupply);
    }

    function testCanRedeemLarge() public {
        uint256 amount = 10e18;
        vm.prank(largeAmktHolder);
        AMKT.delegate(largeAmktHolder);
        vm.roll(block.number + 1);
        // uint256 amount = AMKT.balanceOf(largeAmktHolder);
        // vm.assume(amount < AMKT.balanceOf(largeAmktHolder));
        // vm.warp(block.timestamp + 2 days);
        vault.tryInflation();
        assertEq(AMKT.balanceOf(largeAmktHolder), 16704840500000000000000);
        assertGe(AMKT.totalSupply(), AMKT.balanceOf(largeAmktHolder));
        vm.startPrank(largeAmktHolder);
        console.log(msg.sender);
        console.log(largeAmktHolder);
        AMKT.approve(address(issuance), AMKT.balanceOf(largeAmktHolder));

        issuance.redeem(amount);
        vm.stopPrank();
    }

    function testVaultCanMint() public {
        vm.startPrank(address(vault));
        AMKT.mint(address(vault), 1);
        AMKT.burn(address(vault), 1);
        vm.stopPrank();
    }

    function testUserCanTransfer(uint256 amount) public {
        vm.assume(amount < AMKT.balanceOf(largeAmktHolder));
        vm.startPrank(largeAmktHolder);
        AMKT.transfer(address(vault), amount);
        vm.stopPrank();
    }

    function testRevertOldMinterCanMint() public {
        vm.prank(oldMinter);
        vm.expectRevert();
        AMKT.mint(address(vault), 1);
    }

    function testRevertOldMinterCanBurn() public {
        vm.prank(oldMinter);
        vm.expectRevert();
        AMKT.burn(largeAmktHolder, 1);
    }

    function mint(uint256 amount) public {
        vault.tryInflation();
        Dealer dealer = new Dealer();
        TokenInfo[] memory tokens = vault.realUnits();

        uint256 amountIncludingIntradayInflation = fmul(
            vault.intradayMultiplier(),
            amount
        );

        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            uint256 initialBalance = token.balanceOf(address(this));
            uint256 underlyingAmount = fmul(
                tokens[i].units,
                amountIncludingIntradayInflation
            ) + 1;
            dealer.dealToken(address(token), address(this), underlyingAmount);
            token.approve(address(issuance), token.balanceOf(address(this)));
        }

        issuance.issue(amount);
    }

    function testIssuance(uint256 amount) public {
        vm.assume(amount < 1e25);
        mint(amount);
        TokenInfo[] memory tokens = vault.realUnits();

        // Check the balances of address(this) after issuance
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            assertEq(token.balanceOf(address(this)), 0);
        }

        // check that user can redeem afterwards
        issuance.redeem(amount);
    }

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
