pragma solidity =0.8.18;

import {SCALAR, fmul} from "./FixedPoint.sol";

/// A library to track a compounding fee.
library Multiplier {
    error MultiplierFeeTooHigh();

    /// @notice Computes the multiplier for the current block timestamp
    ///
    /// @param lastTrackedTimestamp the last tracked timestamp
    ///
    /// @param lastTrackedMultiplier the last tracked multiplier, this should be equal to 1 (SCALAR) with the timestamp of the last nominal reset
    ///                              ie rebalances
    ///
    /// @param feeScaled the daily fee scaled
    ///                 ie 2% yearly fee (apy) = (1 - \sqrt[365](1 - .02)) * 1e18
    ///
    /// @return trackedTimestamp the new tracked timestamp
    /// @return trackedMultiplier the new tracked multiplier, this is helpful to cache so we dont need to start from the beginning
    /// @return newFeeAccrued the new fee from this call, does not account for the old tracked multiplier, this does not incude intermediate values and is how inflation accrual works
    /// @return currentMultiplier the new multiplier for the current block timestamp, this is an intermediate value and not tracked, it is whats applied to the nominals
    function computeMultiplier(
        uint256 lastTrackedTimestamp,
        uint256 lastTrackedMultiplier,
        uint256 feeScaled
    )
        internal
        view
        returns (
            uint256 trackedTimestamp,
            uint256 trackedMultiplier,
            uint256 newFeeAccrued,
            uint256 currentMultiplier
        )
    {
        if (feeScaled > SCALAR) {
            revert MultiplierFeeTooHigh();
        }

        trackedTimestamp = lastTrackedTimestamp;

        uint256 oneSubFee = SCALAR - feeScaled;
        uint256 _days = (block.timestamp - trackedTimestamp) / 1 days;

        // newFeeAccrued = (1 - fee) ^ days
        // trackedMultiplier = newFeeAccrued * lastTrackedMultiplier
        newFeeAccrued = SCALAR;
        if (_days > 0) {
            for (uint256 i; i < _days; ) {
                newFeeAccrued = fmul(newFeeAccrued, oneSubFee);

                unchecked {
                    ++i;
                }
            }

            trackedTimestamp += 1 days * _days;
        }

        trackedMultiplier = fmul(newFeeAccrued, lastTrackedMultiplier);

        uint256 dT = block.timestamp - trackedTimestamp;

        if (dT != 0) {
            uint256 feePerSecondScaled = feeScaled / 1 days;

            // currentMultiplier = trackedMultiplier * (1 - feePerSecond * dT)
            currentMultiplier = fmul(
                trackedMultiplier,
                SCALAR - feePerSecondScaled * dT
            );
        } else {
            currentMultiplier = trackedMultiplier;
        }
    }
}
