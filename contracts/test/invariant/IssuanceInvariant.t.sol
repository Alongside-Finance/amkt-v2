pragma solidity =0.8.18;

import {StatefulTest} from "core-test/State.t.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {SCALAR, fmul} from "src/lib/FixedPoint.sol";
import {IVault} from "src/interfaces/IVault.sol";
import {IIssuance} from "src/interfaces/IIssuance.sol";
import {MockMintableToken} from "test/utils/MockMintableToken.sol";

contract IssuanceInvariantTest is StatefulTest {
    function setUp() public override {
        super.setUp();
        seedInitial(10);
        excludeContract(address(indexToken));
    }

    function invariant_issuance_formula() public {
        TokenInfo[] memory units = vault.virtualUnits();
        uint256 issueAmount = 1e18;
        uint256 redeemAmount = 1e18;
        uint256[] memory startingIssuanceVaultBalances = new uint256[](
            units.length
        );
        for (uint256 i; i < units.length; i++) {
            startingIssuanceVaultBalances[i] = IERC20(units[i].token).balanceOf(
                address(vault)
            );
        }

        TokenInfo[] memory tokens = quoter.quoteIssue(issueAmount);
        for (uint256 i = 0; i < tokens.length; i++) {
            MockMintableToken(tokens[i].token).mint(
                address(this),
                tokens[i].units
            );
            IERC20(tokens[i].token).approve(address(issuance), tokens[i].units);
        }

        issuance.issue(issueAmount);

        uint256[] memory endingIssuanceVaultBalances = new uint256[](
            units.length
        );
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

        // indexToken.approve(address(issuance), redeemAmount);
        issuance.redeem(redeemAmount);

        uint256[] memory endingRedemptionVaultBalances = new uint256[](
            units.length
        );
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
}
