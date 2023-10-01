pragma solidity =0.8.18;

import {StatefulTest} from "core-test/State.t.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {SCALAR, fmul} from "src/lib/FixedPoint.sol";
import {IVault} from "src/interfaces/IVault.sol";

contract IssuanceTest is StatefulTest {
    function testGoToZero() public {
        seedInitial(10);
        mint(5e18);
        uint256 totalSupply = indexToken.totalSupply();
        burn(totalSupply);
        assertEq(indexToken.totalSupply(), 0);

        TokenInfo[] memory tokens = vault.realUnits();
        for (uint256 i; i < tokens.length; i++) {
            uint256 actual = IERC20(tokens[i].token).balanceOf(address(vault));
            uint256 target = 1;
            if (actual > target) {
                assertGe(target + 100, actual); // support dust
            } else {
                assertEq(actual, target);
            }
        }
    }

    function testCantGoToZeroWithUnmintedInflation() public {
        // Q: Who keeps the intraday fee on the tokens that are being burnt? The user, vault, or the feeRecipient?
        // A: For now, the vault keeps the intraday fee on the tokens that are being burnt.
        // If this amount becomes meaningful, we can expose a function to withdraw it separately.
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

    function testGoToZeroWithMintedInflation() public {
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
        TokenInfo[] memory tokens = issuanceQuoter.quoteIssue(amountToMint);
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20(tokens[i].token).approve(address(issuance), tokens[i].units);
        }
        vm.warp(oneDayMark - 1);
        issuance.issue(amountToMint);
    }

    function testIssuanceTakesCorrectAmounts() public {
        seedInitial(10);

        vm.warp(block.timestamp + 1 days - 1);

        TokenInfo[] memory quoteUnits = issuanceQuoter.quoteIssue(5e18);
        uint256[] memory startingBalances = new uint256[](quoteUnits.length);
        for (uint256 i; i < quoteUnits.length; i++) {
            startingBalances[i] = IERC20(quoteUnits[i].token).balanceOf(
                address(this)
            );
        }

        mint(5e18);

        for (uint256 i; i < quoteUnits.length; i++) {
            assertEq(
                IERC20(quoteUnits[i].token).balanceOf(address(this)),
                startingBalances[i] - quoteUnits[i].units
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

        TokenInfo[] memory tokens = issuanceQuoter.quoteIssue(5e18);
        TokenInfo[] memory realUnits = vault.realUnits();
        uint256 amountIncludingIntradayInflation = fmul(
            vault.intradayInflation(),
            5e18
        ) + 1;
        for (uint256 i; i < tokens.length; i++) {
            assertEq(
                tokens[i].units,
                fmul(realUnits[i].units + 1, amountIncludingIntradayInflation) +
                    1
            );
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
            assertLe(
                IERC20(realUnits[i].token).balanceOf(address(this)),
                startingBalances[i] // issuance always subtracts
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
