pragma solidity =0.8.18;

import {UpgradedTest} from "test/upgrade/helpers/Upgraded.t.sol";
import {Dealer} from "test/Dealer.t.sol";
import {TokenInfo} from "src/Common.sol";
import {InitialBountyHelper} from "src/scripts/Config.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {console} from "forge-std/console.sol";
import {Quoter} from "periphery/Quoter.sol";
import {IVault} from "src/interfaces/IVault.sol";

contract UpgradedIssuanceTest is UpgradedTest {
    address largeAmktHolder =
        address(0x804B68f60765F4559b7096B158C912eD33aa0c26);
    address oldMinter = address(0x0D44F856E1a7c70E35c54261c3f07DbFBDCA4857);

    function testIssuanceInvariant(
        uint256 issueAmount,
        uint256 redeemAmount
    ) public {
        issueAmount = bound(issueAmount, 0, 10_000_000e18);
        redeemAmount = bound(redeemAmount, 0, issueAmount);
        vm.assume(issueAmount < 10_000_000e18); // we are bound by LDO whale supply
        vm.assume(redeemAmount < issueAmount);
        TokenInfo[] memory units = vault.virtualUnits();
        uint256[] memory startingIssuanceVaultBalances = new uint256[](
            units.length
        );
        for (uint256 i; i < units.length; i++) {
            startingIssuanceVaultBalances[i] = IERC20(units[i].token).balanceOf(
                address(vault)
            );
        }
        assistedMint(address(this), issueAmount);
        uint256[] memory endingIssuanceVaultBalances = new uint256[](
            units.length
        );

        // ISSUANCE INVARIANT:
        for (uint256 i; i < units.length; i++) {
            endingIssuanceVaultBalances[i] = IERC20(units[i].token).balanceOf(
                address(vault)
            );
            assertEq(
                endingIssuanceVaultBalances[i],
                startingIssuanceVaultBalances[i] +
                    fmul(units[i].units + 1, issueAmount) +
                    1
            );
        }
        issuance.redeem(redeemAmount);
        uint256[] memory endingRedemptionVaultBalances = new uint256[](
            units.length
        );

        // REDEMPTION INVARIANT:
        for (uint256 i; i < units.length; i++) {
            endingRedemptionVaultBalances[i] = IERC20(units[i].token).balanceOf(
                address(vault)
            );
            assertEq(
                endingRedemptionVaultBalances[i],
                endingIssuanceVaultBalances[i] -
                    fmul(units[i].units, redeemAmount)
            );
        }
    }

    function testIssuanceWithJitter(
        uint256 issueAmount,
        uint256 jitter
    ) public {
        issueAmount = bound(issueAmount, 0, 10_000_000e18);
        jitter = bound(jitter, 0, JITTER_MAX);
        vm.assume(issueAmount < 10_000_000e18); // we are bound by LDO whale supply
        vm.assume(jitter < JITTER_MAX);
        _warpForward(jitter);
        feeRecipientTryInflation();
        assistedMint(address(this), issueAmount);
        _warpForward(jitter);
        feeRecipientTryInflation();
        assistedMint(address(this), issueAmount);
    }

    function testIssuanceAndRedemptionWithJitter(
        uint256 issueAmount,
        uint256 redeemAmount,
        uint256 jitter
    ) public {
        issueAmount = bound(issueAmount, 0, 10_000_000e18);
        redeemAmount = bound(redeemAmount, 0, issueAmount);
        jitter = bound(jitter, 0, JITTER_MAX);
        vm.assume(issueAmount < 10_000_000e18); // we are bound by LDO whale supply
        vm.assume(redeemAmount <= issueAmount);
        vm.assume(jitter < JITTER_MAX);
        _warpForward(jitter);
        feeRecipientTryInflation();
        assistedMint(address(this), issueAmount);
        TokenInfo[] memory tokens = vault.virtualUnits();

        // Check the balances of address(this) after issuance
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            assertEq(token.balanceOf(address(this)), 0);
        }

        // check that user can redeem afterwards
        _warpForward(jitter);
        feeRecipientTryInflation();
        issuance.redeem(redeemAmount);
    }

    function testTryInflationWithJitter(uint256 jitter) public {
        jitter = bound(jitter, 0, JITTER_MAX);
        vm.assume(jitter < JITTER_MAX);
        _warpForward(jitter);
        feeRecipientTryInflation();
        vm.prank(address(2));
        uint256 beforeSupply = AMKT.totalSupply();
        feeRecipientTryInflation();
        assertEq(AMKT.totalSupply(), beforeSupply);
        _warpForward(jitter);
        feeRecipientTryInflation();
    }

    function testCanRedeemLargeWithJitter(
        uint256 amount,
        uint256 jitter
    ) public {
        amount = bound(amount, 0, AMKT.balanceOf(largeAmktHolder));
        jitter = bound(jitter, 0, JITTER_MAX);
        vm.assume(jitter < JITTER_MAX);
        vm.assume(amount <= AMKT.balanceOf(largeAmktHolder));
        feeRecipientTryInflation();
        vm.startPrank(largeAmktHolder);
        AMKT.approve(address(issuance), AMKT.balanceOf(largeAmktHolder));
        vm.stopPrank();
        _warpForward(jitter);
        feeRecipientTryInflation();
        vm.prank(largeAmktHolder);
        issuance.redeem(amount);
    }

    function testCanRedeemAll() public {
        deal(address(AMKT), address(this), AMKT.totalSupply());
        issuance.redeem(AMKT.totalSupply());
    }

    function testVaultCanMint(uint256 amount) public {
        vm.assume(amount < uint256(type(uint224).max) - 1);
        vm.startPrank(address(vault));
        AMKT.mint(address(vault), amount);
        AMKT.burn(address(vault), amount);
        vm.stopPrank();
    }

    function testUsersCanTransferWithJitter(
        uint256 amount,
        uint256 jitter
    ) public {
        amount = bound(amount, 0, AMKT.balanceOf(largeAmktHolder));
        jitter = bound(jitter, 0, JITTER_MAX);
        vm.assume(jitter < JITTER_MAX);
        vm.assume(amount <= AMKT.balanceOf(largeAmktHolder));
        vm.startPrank(largeAmktHolder);
        AMKT.transfer(address(3), amount);
        vm.stopPrank();
        _warpForward(jitter);
        feeRecipientTryInflation();
        vm.startPrank(address(3));
        AMKT.transfer(address(4), amount);
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

    function testIssuance(uint256 issueAmount, uint256 redeemAmount) public {
        issueAmount = bound(issueAmount, 0, 10_000_000e18);
        redeemAmount = bound(redeemAmount, 0, issueAmount);
        vm.assume(issueAmount < 10_000_000e18); // we are bound by LDO whale supply
        vm.assume(redeemAmount <= issueAmount);
        assistedMint(address(this), issueAmount);
        TokenInfo[] memory tokens = vault.virtualUnits();

        // Check the balances of address(this) after issuance
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            assertEq(token.balanceOf(address(this)), 0);
        }

        // check that user can redeem afterwards
        issuance.redeem(redeemAmount);
    }

    // Helpers

    function assistedMint(address to, uint256 amount) internal {
        Quoter quoter = new Quoter(address(vault));
        Dealer dealer = new Dealer();
        TokenInfo[] memory tokens = quoter.quoteIssue(amount);
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            dealer.dealToken(address(token), to, tokens[i].units);
            token.approve(address(issuance), tokens[i].units);
        }
        issuance.issue(amount);
    }

    function feeRecipientTryInflation() public {
        vm.startPrank(vault.feeRecipient());
        uint256 beforeSupply = AMKT.totalSupply();
        if (block.timestamp - vault.lastKnownTimestamp() <= 1 days) {
            vm.expectRevert(IVault.AMKTVaultFeeTooEarly.selector);
            vault.tryInflation();
            uint256 afterSupply = AMKT.totalSupply();
            assertEq(beforeSupply, afterSupply);
        } else {
            vault.tryInflation();
            uint256 afterSupply = AMKT.totalSupply();
            assertGe(afterSupply, beforeSupply);
        }
        vm.stopPrank();
    }
}
