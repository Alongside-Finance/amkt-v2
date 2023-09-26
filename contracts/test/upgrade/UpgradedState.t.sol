pragma solidity =0.8.15;

import "forge-std/Test.sol";
import {InitialBountyHelper} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {Dealer} from "test/Dealer.t.sol";
import {UpgradeTest} from "./Upgrade.t.sol";

contract UpgradedState is UpgradeTest {
    address largeAmktHolder =
        address(0x804B68f60765F4559b7096B158C912eD33aa0c26);
    address oldMinter = address(0x0D44F856E1a7c70E35c54261c3f07DbFBDCA4857);

    function testState() public {
        assertEq(
            address(AMKT),
            address(0xF17A3fE536F8F7847F1385ec1bC967b2Ca9caE8D)
        );
        assertEq(AMKT.minter(), address(vault));
        assertEq(AMKT.decimals(), 18);
        assertEq(AMKT.symbol(), "AMKT");
        assertEq(AMKT.name(), "Alongside Crypto Market Index");
        assertEq(vault.underlyingLength(), 15); // TODO: Increase to 15
        assertEq(vault.issuance(), address(issuance));
        assertEq(vault.rebalancer(), address(timelockInvokeableBounty));
        assertEq(
            vault.feeRecipient(),
            0xC19a5b6E0a923519603985153515222D59cb3F2e
        );
        assertEq(
            vault.emergencyResponder(),
            address(0xAeB9ef94b6542BE7112f3a295646B5AaAa9Fca13)
        );
        assertEq(vault.emergency(), false);
        assertEq(address(vault.indexToken()), address(AMKT));
        assertEq(vault.feeScaled(), 26151474053915);
        assertEq(address(timelockInvokeableBounty.indexToken()), address(AMKT));
        assertEq(address(timelockInvokeableBounty.vault()), address(vault));
        assertEq(
            address(timelockInvokeableBounty.activeBounty()),
            address(timelockActiveBounty)
        );
        assertEq(timelockInvokeableBounty.version, 2);
        assertEq(timelockInvokeableBounty.chainId, 1);
        assertEq(
            timelockActiveBounty.authority,
            address(0xAeB9ef94b6542BE7112f3a295646B5AaAa9Fca13)
        );
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

    function testTryInflation() public {
        vm.prank(address(2));
        uint256 beforeSupply = AMKT.totalSupply();
        vault.tryInflation();
        assertEq(AMKT.totalSupply(), beforeSupply);

        vm.warp(block.timestamp + 1 days);
        vault.tryInflation();
        assertGe(AMKT.totalSupply(), beforeSupply);
    }

    function testVaultCanMint() public {
        vm.startPrank(address(vault));
        AMKT.mint(address(vault), 1);
        AMKT.burn(address(vault), 1);
    }

    function testUserCanTransfer() public {
        vm.startPrank(largeAmktHolder);
        AMKT.transfer(address(vault), 1);
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

    function testIssuance() public {
        Dealer dealer = new Dealer();
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();

        uint256 initialDealtAmount = 5;
        uint256 issuedAmount = 5;

        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            uint256 initialBalance = token.balanceOf(address(this));
            dealer.dealToken(
                address(token),
                address(this),
                (initialDealtAmount * tokens[i].units) / 1e18 + 1
            );
            token.approve(address(issuance), token.balanceOf(address(this)));
        }

        issuance.issue(issuedAmount);

        // Check the balances of address(this) after issuance
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            assertEq(
                token.balanceOf(address(this)),
                initialDealtAmount - issuedAmount
            );
            assertEq(token.balanceOf(address(this)), 0);
        }

        // TODO: Check vault banaces
    }

    function testNeqErc20PermitDomainSeparator() public {
        bytes32 typeHash = keccak256(
            "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
        );
        bytes32 nameHash = keccak256(bytes(AMKT.name()));
        bytes32 versionHash = keccak256(bytes("1"));
        uint256 chainId = 1;
        address token = address(AMKT);
        bytes32 expectedDomainSeparator = keccak256(
            abi.encode(typeHash, nameHash, versionHash, chainId, token)
        );
        assertNotEq(AMKT.DOMAIN_SEPARATOR(), expectedDomainSeparator);
    }

    function testEqErc20PermitDomainSeparator() public {
        bytes32 typeHash = keccak256(
            "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
        );
        bytes32 nameHash = keccak256(bytes(AMKT.name()));
        bytes32 versionHash = keccak256(bytes("2"));
        uint256 chainId = 1;
        address token = address(AMKT);
        bytes32 expectedDomainSeparator = keccak256(
            abi.encode(typeHash, nameHash, versionHash, chainId, token)
        );
        assertEq(AMKT.DOMAIN_SEPARATOR(), expectedDomainSeparator);
    }
}
