pragma solidity =0.8.18;

import {StatefulTest} from "test/core/State.t.sol";
import {SymTest} from "halmos-cheatcodes/SymTest.sol";
import {fmul, fdiv} from "src/lib/FixedPoint.sol";
import {SymbolicStatefulTest} from "./SymbolicState.t.sol";
import {FEE_SCALED} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";

contract SymbolicVaultTest is SymbolicStatefulTest {
    function check_inflation(
        uint256 secondsPassed,
        uint256 inflationRate
    ) public {
        vm.assume(secondsPassed < 365 days * 2);
        vm.assume(inflationRate < 1e18);
        uint256 startingSupply = indexToken.totalSupply();
        uint256 inflation = fmul(startingSupply, secondsPassed * inflationRate);
        inflationTestHelper(secondsPassed, inflation);
    }

    function inflationTestHelper(
        uint256 secondsPassed,
        uint256 expectedInflation
    ) internal {
        seedInitial(3);
        uint256 initialSupply = indexToken.totalSupply();
        uint256 initialFeeRecipientBalance = indexToken.balanceOf(feeReciever);
        TokenInfo[] memory initialUnits = vault.virtualUnits();
        vm.warp(block.timestamp + secondsPassed);
        vm.prank(feeReciever);
        vault.tryInflation();
        uint256 newSupply = indexToken.totalSupply();
        uint256 newFeeRecipientBalance = indexToken.balanceOf(feeReciever);
        uint256 expectedSupply = initialSupply + expectedInflation;
        assertEq(newSupply, expectedSupply);
        assertEq(newFeeRecipientBalance, expectedInflation);
        uint256 valueMultiplier = fdiv(
            initialSupply,
            initialSupply + expectedInflation
        );
        TokenInfo[] memory newUnits = vault.virtualUnits();
        for (uint256 i = 0; i < newUnits.length; i++) {
            assertEq(
                newUnits[i].units,
                fmul(initialUnits[i].units, valueMultiplier)
            );
        }
    }
}
