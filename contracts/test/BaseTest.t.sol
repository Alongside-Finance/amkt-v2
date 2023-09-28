import {Test} from "forge-std/Test.sol";

contract BaseTest is Test {
    uint256 AVG_BLOCK_TIME = 12;

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

    function resetTimeAndBlock(
        uint256 timestamp,
        uint256 blockNumber
    ) internal {
        vm.warp(timestamp);
        vm.roll(blockNumber);
    }

    function warpForward(uint256 secondsToMove) internal {
        vm.warp(block.timestamp + secondsToMove);
        vm.roll(block.number + secondsToMove / 12);
    }
}
