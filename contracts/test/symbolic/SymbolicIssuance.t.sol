pragma solidity =0.8.18;

import {StatefulTest} from "test/core/State.t.sol";
import {SymTest} from "halmos-cheatcodes/SymTest.sol";
import {fmul, fdiv} from "src/lib/FixedPoint.sol";
import {SymbolicStatefulTest} from "./SymbolicState.t.sol";
import {INFLATION_RATE} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";

contract SymbolicIssuanceTest is SymbolicStatefulTest {
    function check_issuance_formula(
        uint256 issueAmount,
        uint256 redeemAmount
    ) public {
        seedInitial(3);
        vm.assume(issueAmount < 1e25);
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
        mint(issueAmount);
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
        burn(redeemAmount);
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
