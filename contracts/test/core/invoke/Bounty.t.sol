pragma solidity =0.8.18;

import {StatefulTest, Mocks} from "core-test/State.t.sol";
import {InvokeableBounty, Bounty} from "invoke-modules/Bounty.sol";
import {MockMintableToken} from "mocks/MockMintableToken.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {SCALAR} from "src/lib/FixedPoint.sol";

interface Rebalancer {
    function rebalanceCallback(
        TokenInfo[] calldata x,
        TokenInfo[] calldata y
    ) external;
}

contract BountyTest is StatefulTest {
    Bounty internal bountyHolder;
    bool internal reenter;
    bool internal mintOnCallback;

    function testInitialBounty(uint256 quantity) public {
        TokenInfo[] memory tokens = seedInitial(quantity);

        // assert the multiplier has been resset
        (, , uint256 current, ) = vault.multiplier();
        assertEq(current, SCALAR);

        // seedInitial() just uses acsending token mock
        for (uint256 i = 0; i < tokens.length; i++) {
            address token = tokens[i].token;

            // assert the proper balance is included
            assertEq(
                IERC20(address(token)).balanceOf(address(vault)),
                (i + 1) * SCALAR
            );

            // assert the token was marked as underlying
            assertEq(vault.isUnderlying(token), true);
        }
    }

    function testRebalance() public {
        uint256 oneDayMark = block.timestamp + 1 days;
        TokenInfo[] memory tokens = seedInitial(15);
        TokenInfo[] memory newTokens = tokens;

        for (uint256 i = 0; i < newTokens.length; i++) {
            newTokens[i].units = (newTokens[i].units + 1);
            IERC20(address(tokens[i].token)).approve(
                address(bounty),
                type(uint256).max
            );
            MockMintableToken(address(tokens[i].token)).mint(
                address(this),
                100e18
            );
        }

        Bounty memory _bounty = Bounty({
            infos: newTokens,
            salt: keccak256("test"),
            deadline: block.timestamp + 2 days
        });

        bytes32 _hash = bounty.hashBounty(_bounty);

        vm.prank(authority);
        activeBounty.setHash(_hash);
        vm.warp(oneDayMark - 1);
        bounty.fulfillBounty(_bounty, true);
    }

    function testRemoveToken() public {
        TokenInfo[] memory tokens = seedInitial(5);

        TokenInfo[] memory newTokensNominals = tokens;
        newTokensNominals[0].units = 0;

        for (uint256 i; i < newTokensNominals.length; i++) {
            IERC20(address(tokens[i].token)).approve(
                address(bounty),
                type(uint256).max
            );
        }

        vm.warp(2 days);

        Bounty memory _bounty = Bounty({
            infos: newTokensNominals,
            salt: keccak256("test"),
            deadline: block.timestamp + 1000
        });

        bytes32 _hash = bounty.hashBounty(_bounty);

        vm.prank(authority);
        activeBounty.setHash(_hash);

        vm.warp(block.timestamp + 1001);
        vm.expectRevert();
        bounty.fulfillBounty(_bounty, true);
        vm.warp(2 days);

        bounty.fulfillBounty(_bounty, true);

        assertEq(vault.isUnderlying(newTokensNominals[0].token), false);
        assertEq(
            IERC20(address(tokens[0].token)).balanceOf(address(vault)) <= 1,
            true
        );
    }

    function testfulfillBountyPreventsReEntranncy() public {
        reenter = true;

        TokenInfo[] memory tokens = Mocks.ascendingTokenNominalsMock(
            address(bounty),
            5
        );
        Bounty memory _bounty = Mocks.bountyMock(tokens);
        holdBounty(_bounty);
        validateBounty(_bounty);
        vm.expectRevert(InvokeableBounty.BountyReentrant.selector);
        bounty.fulfillBounty(_bounty, true);
    }

    function testInvalidBountyHash() public {
        TokenInfo[] memory tokens = seedInitial(5);
        Bounty memory invalidBounty = Bounty({
            infos: tokens,
            deadline: block.timestamp + 1000,
            salt: keccak256("invalid")
        });

        vm.expectRevert(InvokeableBounty.BountyInvalidHash.selector);
        bounty.fulfillBounty(invalidBounty, true);
    }

    function testBountyAlreadyCompleted() public {
        TokenInfo[] memory tokens = seedInitial(5);
        Bounty memory _bounty = Bounty({
            infos: tokens,
            deadline: block.timestamp + 1000,
            salt: keccak256("test")
        });

        holdBounty(_bounty);
        validateBounty(_bounty);

        bounty.fulfillBounty(_bounty, true);

        vm.expectRevert(InvokeableBounty.BountyAlreadyCompleted.selector);
        bounty.fulfillBounty(_bounty, true);
    }

    function testBountyMustIncludeAllUnderlyings() public {
        TokenInfo[] memory tokens = seedInitial(5);

        // Remove one token from the bounty
        TokenInfo[] memory newTokens = new TokenInfo[](tokens.length - 1);
        for (uint256 i = 0; i < newTokens.length; i++) {
            newTokens[i] = tokens[i];
        }

        Bounty memory _bounty = Bounty({
            infos: newTokens,
            deadline: block.timestamp + 1000,
            salt: keccak256("test")
        });

        holdBounty(_bounty);
        validateBounty(_bounty);

        vm.expectRevert(
            InvokeableBounty.BountyMustIncludeAllUnderlyings.selector
        );
        bounty.fulfillBounty(_bounty, true);
    }

    function testFailBountyAMKTSupplyChange() public {
        mintOnCallback = true;
        TokenInfo[] memory tokens = seedInitial(5);
        Bounty memory _bounty = Bounty({
            infos: tokens,
            deadline: block.timestamp + 1000,
            salt: keccak256("test")
        });

        holdBounty(_bounty);
        validateBounty(_bounty);

        bounty.fulfillBounty(_bounty, true);
    }

    function testQuoteBounty() public {
        TokenInfo[] memory tokens = seedInitial(5);
        Bounty memory _bounty = Bounty({
            infos: tokens,
            deadline: block.timestamp + 1000,
            salt: keccak256("test")
        });

        (TokenInfo[] memory outs, TokenInfo[] memory ins) = bounty.quoteBounty(
            _bounty
        );

        uint256 lenOuts = outs.length;
        uint256 lenIns = ins.length;

        for (uint256 i = 0; i < lenOuts; i++) {
            assertEq(outs[i].token, tokens[i].token);
            assertEq(outs[i].units, tokens[i].units);
        }

        for (uint256 i = 0; i < lenIns; i++) {
            assertEq(ins[i].token, tokens[lenOuts + i].token);
            assertEq(ins[i].units, tokens[lenOuts + i].units);
        }
    }

    function rebalanceCallback(
        TokenInfo[] calldata x,
        TokenInfo[] calldata y
    ) external override {
        if (reenter) {
            bounty.fulfillBounty(bountyHolder, true);
        }
        if (mintOnCallback) {
            indexToken.mint(address(this), 1000 ether);
        }
    }

    function holdBounty(Bounty memory _bounty) internal {
        bountyHolder.deadline = _bounty.deadline;
        bountyHolder.salt = _bounty.salt;
        for (uint256 i = 0; i < _bounty.infos.length; i++) {
            bountyHolder.infos.push(_bounty.infos[i]);
        }
    }
}
