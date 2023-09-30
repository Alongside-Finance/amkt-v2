pragma solidity =0.8.18;

import {SCALAR, fmul} from "./FixedPoint.sol";

/// A library to track a compounding fee.
library Multiplier {
    error MultiplierFeeTooHigh();

    // TODO: new natspec
    function computeMultiplier(
        uint256 lastTrackedTimestamp,
        uint256 lastTrackedMultiplier,
        uint256 feeScaled
    )
        internal
        view
        returns (
            uint256 feeToAccrue,
            uint256 currentTimestamp,
            uint256 currentMultiplier
        )
    {
        if (feeScaled > SCALAR) {
            revert MultiplierFeeTooHigh();
        }
        if (block.timestamp < lastTrackedTimestamp) {
            revert MultiplierFeeTooHigh();
        }

        uint256 timestampDiff = block.timestamp - lastTrackedTimestamp;
        feeToAccrue = SCALAR - feeScaled * timestampDiff;
        currentMultiplier = fmul(lastTrackedMultiplier, feeToAccrue);
        currentTimestamp = block.timestamp;
    }
}
