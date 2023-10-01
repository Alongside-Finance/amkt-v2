pragma solidity =0.8.18;

import {UpgradeTest} from "./helpers/Upgrade.t.sol";
import {Dealer} from "test/Dealer.t.sol";
import {TokenInfo} from "src/Common.sol";
import {InitialBountyHelper} from "src/scripts/Config.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {console} from "forge-std/console.sol";
import {IssuanceQuoter} from "periphery/IssuanceQuoter.sol";

contract UpgradedIssuanceTest is UpgradeTest {
    address largeAmktHolder =
        address(0x804B68f60765F4559b7096B158C912eD33aa0c26);
    address oldMinter = address(0x0D44F856E1a7c70E35c54261c3f07DbFBDCA4857);

    /// forge-config: default.fuzz.runs = 2048
    function testIssuanceWithJitter(
        uint256 issueAmount,
        uint256 jitter
    ) public {
        console.log("issueAmount: %s", issueAmount);
        console.log("jitter: %s", jitter);
        issueAmount = bound(issueAmount, 0, 10_000_000e18);
        vm.assume(issueAmount < 10_000_000e18); // we are bound by LDO whale supply
        vm.assume(jitter < 365 days);
        warpForward(jitter);
        assistedMint(address(this), issueAmount);
        warpForward(jitter);
        assistedMint(address(this), issueAmount);
    }

    /// forge-config: default.fuzz.runs = 2048
    function testIssuanceAndRedemptionWithJitter(
        uint256 issueAmount,
        uint256 redeemAmount,
        uint256 jitter
    ) public {
        console.log("issueAmount: %s", issueAmount);
        console.log("redeemAmount: %s", redeemAmount);
        console.log("jitter: %s", jitter);
        issueAmount = bound(issueAmount, 0, 10_000_000e18);
        redeemAmount = bound(redeemAmount, 0, issueAmount);
        vm.assume(issueAmount < 10_000_000e18); // we are bound by LDO whale supply
        vm.assume(redeemAmount <= issueAmount);
        vm.assume(jitter < 365 days);
        warpForward(jitter);
        assistedMint(address(this), issueAmount);
        TokenInfo[] memory tokens = vault.realUnits();

        // Check the balances of address(this) after issuance
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            assertEq(token.balanceOf(address(this)), 0);
        }

        // check that user can redeem afterwards
        warpForward(jitter);
        issuance.redeem(redeemAmount);
    }

    function testTryInflation() public {
        vault.tryInflation();
        vm.prank(address(2));
        uint256 beforeSupply = AMKT.totalSupply();
        vault.tryInflation();
        assertEq(AMKT.totalSupply(), beforeSupply);

        vm.warp(block.timestamp + 1);
        vault.tryInflation();
        assertGe(AMKT.totalSupply(), beforeSupply);
    }

    function testCanRedeemLarge(uint256 amount) public {
        vm.assume(amount <= AMKT.balanceOf(largeAmktHolder));
        vault.tryInflation();
        assertEq(AMKT.balanceOf(largeAmktHolder), 16704840500000000000000);
        assertGe(AMKT.totalSupply(), AMKT.balanceOf(largeAmktHolder));
        vm.startPrank(largeAmktHolder);
        AMKT.approve(address(issuance), AMKT.balanceOf(largeAmktHolder));
        issuance.redeem(amount);
        vm.stopPrank();
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

    function testUserCanTransfer(uint256 amount) public {
        vm.assume(amount <= AMKT.balanceOf(largeAmktHolder));
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

    function testIssuance(uint256 issueAmount, uint256 redeemAmount) public {
        vm.assume(issueAmount < 10_000_000e18); // we are bound by LDO whale supply
        vm.assume(redeemAmount <= issueAmount);
        assistedMint(address(this), issueAmount);
        TokenInfo[] memory tokens = vault.realUnits();

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
        IssuanceQuoter issuanceQuoter = new IssuanceQuoter(address(vault));
        vault.tryInflation();
        Dealer dealer = new Dealer();
        TokenInfo[] memory tokens = issuanceQuoter.quoteIssue(amount);
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            dealer.dealToken(address(token), to, tokens[i].units);
            token.approve(address(issuance), tokens[i].units);
        }
        issuance.issue(amount);
    }
}
