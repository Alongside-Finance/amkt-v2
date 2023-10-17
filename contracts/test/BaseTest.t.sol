pragma solidity =0.8.18;

import {Test} from "forge-std/Test.sol";

contract BaseTest is Test {
    uint256 AVG_BLOCK_TIME = 12;
    uint256 MAX_JITTER = 1000 days;

    function rangeCheck(
        uint256 target,
        uint256 actual,
        uint256 rangeNumerator,
        uint256 rangeDenominator
    ) internal {
        uint256 spread = (target * rangeNumerator) / rangeDenominator;
        uint256 upper = target + spread;
        uint256 lower = target - spread;

        assertLt(actual, upper);
        assertGt(actual, lower);

        require(actual <= upper && actual >= lower, "rangeCheck");
    }

    function _resetTimeAndBlock(
        uint256 timestamp,
        uint256 blockNumber
    ) internal {
        vm.warp(timestamp);
        vm.roll(blockNumber);
    }

    function _warpForward(uint256 secondsToMove) internal {
        vm.warp(block.timestamp + secondsToMove);
        vm.roll(block.number + secondsToMove / 12);
    }
}
