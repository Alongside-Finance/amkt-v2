pragma solidity =0.8.15;

import {StatefulTest} from "core-test/State.t.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {SCALAR} from "src/lib/FixedPoint.sol";
import {IVault} from "src/interfaces/IVault.sol";

contract IssuanceTest is StatefulTest {
    function testToZero() public {
        seedInitial(10);
        mint(5e18);
        burn(indexToken.totalSupply());
        address[] memory underlying = vault.underlying();
        assertEq(indexToken.totalSupply(), 0);
        for (uint256 i; i < underlying.length; i++) {
            assertEq(IERC20(underlying[i]).balanceOf(address(vault)), 1);
        }
    }

    function testToZeroWithUnmintedInflation() public {
        vault.setFeeRecipient(address(this));
        seedInitial(10);
        mint(5e18);
        vm.warp(block.timestamp + 1 days - 1);
        burn(indexToken.totalSupply());
        assertGe(indexToken.totalSupply(), 0);
        burn(indexToken.totalSupply());
        address[] memory underlying = vault.underlying();
        assertEq(indexToken.totalSupply(), 0);
        for (uint256 i; i < underlying.length; i++) {
            assertGe(IERC20(underlying[i]).balanceOf(address(vault)), 100);
        }
    }

    function testToZeroWithMintedInflation() public {
        vault.setFeeRecipient(address(this));
        seedInitial(10);
        mint(5e18);
        vm.warp(block.timestamp + 1 days);
        burn(indexToken.totalSupply());
        assertGe(indexToken.totalSupply(), 0);
        burn(indexToken.totalSupply());
        address[] memory underlying = vault.underlying();
        assertEq(indexToken.totalSupply(), 0);
        for (uint256 i; i < underlying.length; i++) {
            assertLe(IERC20(underlying[i]).balanceOf(address(vault)), 100);
        }
    }

    function testShouldMintWithApprovedTokens() public {
        seedInitial(10);

        vm.warp(block.timestamp + 1 days);

        uint256 startingBalance = indexToken.balanceOf(address(this));

        mint(5e18);

        assertEq(indexToken.balanceOf(address(this)), startingBalance + 5e18);
    }

    function testRevertShouldNotMintWithNotApprovedTokens() public {
        seedInitial(10);
        vm.expectRevert();
        issuance.issue(5e18);
    }

    function testIssuanceShouldNotBreakIntraday() public {
        uint256 amountToMint = 5e18;
        uint256 oneDayMark = block.timestamp + 1 days;
        seedInitial(10);
        TokenInfo[] memory tokens = issuance.quote(amountToMint);
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20(tokens[i].token).approve(address(issuance), tokens[i].units);
        }
        vm.warp(oneDayMark - 1);
        issuance.issue(amountToMint);
    }

    function testIssuanceTakesCorrectAmounts() public {
        seedInitial(10);

        vm.warp(block.timestamp + 1 days);

        TokenInfo[] memory realUnits = vault.realUnits();
        uint256[] memory startingBalances = new uint256[](realUnits.length);
        for (uint256 i; i < realUnits.length; i++) {
            startingBalances[i] = IERC20(realUnits[i].token).balanceOf(
                address(this)
            );
        }

        mint(5e18);

        for (uint256 i; i < realUnits.length; i++) {
            assertEq(
                IERC20(realUnits[i].token).balanceOf(address(this)) + 1,
                startingBalances[i] - (realUnits[i].units * 5e18) / SCALAR
            );
        }
    }

    function testShouldMintWhenNotEmergency() public {
        seedInitial(10);
        mint(5e18);
    }

    function testFailShouldNotMintWhenEmergency() public {
        seedInitial(10);
        vm.prank(emergencyResponder);
        vault.setEmergency(true);
        mint(5e18);
    }

    function testFailIssueWithNoTokensInVault() public {
        seedInitial(0);
        mint(5e18);
    }

    function testFailRedeemWithNoTokensInVault() public {
        seedInitial(0);
        burn(5e18);
    }

    function testRedeem() public {
        seedInitial(10);

        vm.warp(block.timestamp + 1 days);

        uint256 startingBalance = indexToken.balanceOf(address(this));

        // Mint some tokens first
        mint(5e18);

        // Redeem tokens
        burn(2e18);

        assertEq(indexToken.balanceOf(address(this)), startingBalance + 3e18);
    }

    function testRedeemCorrectUnderlyingAmounts() public {
        seedInitial(10);

        vm.warp(block.timestamp + 1 days);

        // Mint some tokens first
        mint(5e18);

        TokenInfo[] memory realUnits = vault.realUnits();
        uint256[] memory startingBalances = new uint256[](realUnits.length);
        for (uint256 i; i < realUnits.length; i++) {
            startingBalances[i] = IERC20(realUnits[i].token).balanceOf(
                address(this)
            );
        }

        // Redeem tokens
        burn(2e18);

        for (uint256 i; i < realUnits.length; i++) {
            assertEq(
                IERC20(realUnits[i].token).balanceOf(address(this)),
                startingBalances[i] + (realUnits[i].units * 2e18) / SCALAR
            );
        }
    }

    function testQuote() public {
        seedInitial(10);

        TokenInfo[] memory tokens = issuance.quote(5e18);
        TokenInfo[] memory realUnits = vault.realUnits();

        for (uint256 i; i < tokens.length; i++) {
            assertEq(tokens[i].units, (realUnits[i].units * 5e18) / SCALAR + 1);
        }
    }

    function testIssueWithNoUnderlyingAmount() public {
        seedInitial(10);

        TokenInfo[] memory realUnits = vault.realUnits();
        uint256[] memory startingBalances = new uint256[](realUnits.length);
        for (uint256 i; i < realUnits.length; i++) {
            startingBalances[i] = IERC20(realUnits[i].token).balanceOf(
                address(this)
            );
        }

        mint(0);

        for (uint256 i; i < realUnits.length; i++) {
            assertEq(
                IERC20(realUnits[i].token).balanceOf(address(this)),
                startingBalances[i] - 1 // issuance always subtracts one
            );
        }
    }

    function testRedeemWithNoUnderlyingAmount() public {
        seedInitial(10);

        // Mint some tokens first
        mint(5e18);

        TokenInfo[] memory realUnits = vault.realUnits();
        uint256[] memory startingBalances = new uint256[](realUnits.length);
        for (uint256 i; i < realUnits.length; i++) {
            startingBalances[i] = IERC20(realUnits[i].token).balanceOf(
                address(this)
            );
        }

        // Redeem tokens with no underlying amount
        burn(0);

        for (uint256 i; i < realUnits.length; i++) {
            assertEq(
                IERC20(realUnits[i].token).balanceOf(address(this)),
                startingBalances[i]
            );
        }
    }
}
