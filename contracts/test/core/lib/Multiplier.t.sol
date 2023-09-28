pragma solidity =0.8.18;

import {console} from "forge-std/console.sol";
import {Multiplier} from "src/lib/Multiplier.sol";
import {SCALAR} from "src/lib/FixedPoint.sol";
import "forge-std/Test.sol";

// A helper contract is needed for forge to pick up coverage for libraries
contract MultiplierHelper {
    function computeMultiplier(
        uint256 lastTrackedTimestamp,
        uint256 lastTrackedMultiplier,
        uint256 feeScaled
    )
        external
        view
        returns (
            uint256 trackedTimestamp,
            uint256 trackedMultiplier,
            uint256 newFeeAccrued,
            uint256 currentMultiplier
        )
    {
        (
            uint256 trackedTimestamp,
            uint256 trackedMultiplier,
            uint256 newFeeAccrued,
            uint256 currentMultiplier
        ) = Multiplier.computeMultiplier(
                lastTrackedTimestamp,
                lastTrackedMultiplier,
                feeScaled
            );
        return (
            trackedTimestamp,
            trackedMultiplier,
            newFeeAccrued,
            currentMultiplier
        );
    }
}

contract MultiplierTest is Test {
    MultiplierHelper multiplierHelper = new MultiplierHelper();

    function testBasicFeeAccrual() public {
        vm.warp(100 days); // set block.timestamp to non-zero
        uint256 lastTrackedTimestamp = block.timestamp - 1 days;
        uint256 lastTrackedMultiplier = SCALAR; // Assuming SCALAR represents 1 in the fixed point representation
        uint256 feeScaled = 1234;

        (
            uint256 newTrackedTimestamp,
            uint256 newTrackedMultiplier,
            ,

        ) = multiplierHelper.computeMultiplier(
                lastTrackedTimestamp,
                lastTrackedMultiplier,
                feeScaled
            );

        assertEq(newTrackedTimestamp, block.timestamp);
        assertTrue(newTrackedMultiplier < SCALAR);
    }

    function testZeroDaysPassed() public {
        uint256 lastTrackedTimestamp = block.timestamp;
        uint256 lastTrackedMultiplier = SCALAR;
        uint256 feeScaled = 1234; // 2% fee

        (uint256 newTrackedTimestamp, , uint256 newFeeAccrued, ) = Multiplier
            .computeMultiplier(
                lastTrackedTimestamp,
                lastTrackedMultiplier,
                feeScaled
            );

        assertEq(newTrackedTimestamp, block.timestamp);
        assertEq(newFeeAccrued, SCALAR);
    }

    function testHighFeeShouldRevert() public {
        uint256 lastTrackedTimestamp = block.timestamp;
        uint256 lastTrackedMultiplier = SCALAR;
        uint256 feeScaled = SCALAR + 1; // Fee greater than SCALAR

        vm.expectRevert();
        multiplierHelper.computeMultiplier(
            lastTrackedTimestamp,
            lastTrackedMultiplier,
            feeScaled
        );
    }

    function testFeeAccruedOverMultipleDays() public {
        vm.warp(100 days); // set block.timestamp to non-zero
        uint256 daysPassed = 5;
        uint256 lastTrackedTimestamp = block.timestamp - daysPassed * 1 days;
        uint256 lastTrackedMultiplier = SCALAR;
        uint256 feeScaled = 1234;

        (
            uint256 newTrackedTimestamp,
            ,
            uint256 newFeeAccrued,

        ) = multiplierHelper.computeMultiplier(
                lastTrackedTimestamp,
                lastTrackedMultiplier,
                feeScaled
            );

        assertEq(
            newTrackedTimestamp,
            block.timestamp - (block.timestamp % 1 days)
        );
        assertTrue(newFeeAccrued < SCALAR);
    }

    function testComputeMultiplierWithNonZero_dT() public {
        // Setting up initial parameters
        uint256 lastTrackedTimestamp = 1234567890; // Mock timestamp value
        uint256 lastTrackedMultiplier = SCALAR;
        uint256 feeScaled = SCALAR / 100; // 1% fee for simplicity

        vm.warp(lastTrackedTimestamp + 5000);

        // Call the computeMultiplier function
        (, , , uint256 currentMultiplier) = multiplierHelper.computeMultiplier(
            lastTrackedTimestamp,
            lastTrackedMultiplier,
            feeScaled
        );

        // Calculating the expected currentMultiplier manually
        uint256 dT = 5000;
        uint256 feePerSecondScaled = feeScaled / 1 days;
        uint256 expectedCurrentMultiplier = (lastTrackedMultiplier *
            (SCALAR - feePerSecondScaled * dT)) / SCALAR; // Ensure division by SCALAR for fixed point operations

        // Asserting that the currentMultiplier is as expected
        assertEq(currentMultiplier, expectedCurrentMultiplier);
    }
}
