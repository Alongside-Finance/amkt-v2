pragma solidity =0.8.15;

import {UpgradeTest} from "./helpers/Upgrade.t.sol";
import {Dealer} from "test/Dealer.t.sol";
import {TokenInfo} from "src/Common.sol";
import {InitialBountyHelper} from "src/scripts/Config.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {console} from "forge-std/console.sol";

contract UpgradedFunctionalityTest is UpgradeTest {
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

    function testVaultCanMint(uint256 amount) public {
        vm.assume(amount < uint256(type(uint224).max));
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

    function testIssuance(uint256 amount) public {
        vm.assume(amount < 1e25);
        assistedMint(address(this), amount);
        TokenInfo[] memory tokens = vault.realUnits();

        // Check the balances of address(this) after issuance
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            assertEq(token.balanceOf(address(this)), 0);
        }

        // check that user can redeem afterwards
        issuance.redeem(amount);
    }

    // Helpers

    function assistedMint(address to, uint256 amount) internal {
        vault.tryInflation();
        Dealer dealer = new Dealer();
        TokenInfo[] memory tokens = vault.realUnits();

        uint256 amountIncludingIntradayInflation = fmul(
            vault.intradayMultiplier(),
            amount
        );

        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            uint256 initialBalance = token.balanceOf(to);
            uint256 underlyingAmount = fmul(
                tokens[i].units,
                amountIncludingIntradayInflation
            ) + 1;
            dealer.dealToken(address(token), to, underlyingAmount);
            token.approve(address(issuance), underlyingAmount);
        }

        issuance.issue(amount);
    }
}
