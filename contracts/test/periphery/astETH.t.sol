import {astETH} from "periphery/astETH.sol";
import {BaseTest} from "test/utils/BaseTest.t.sol";
import {MockMintableToken} from "test/utils/MockMintableToken.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

// 0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84 stETH on mainnet

contract astETHTest is BaseTest {
    astETH token;
    MockMintableToken stETH;

    function setUp() public {
        stETH = new MockMintableToken("stETH", "stETH", 18, 0);
        token = new astETH(
            IERC20(address(stETH)),
            address(this),
            address(this)
        );
    }

    function testDeposit(uint256 amount) public {
        _mintStETHAndDeposit(amount);
        assertEq(token.balanceOf(address(this)), amount);
    }

    function testWithdraw(uint256 amount) public {
        _mintStETHAndDeposit(amount);
        token.withdraw(amount);
        assertEq(token.balanceOf(address(this)), 0);
    }

    function testCollectFee(uint256 supply, uint256 surplus) public {
        supply = bound(supply, 0, type(uint128).max);
        surplus = bound(surplus, 0, type(uint128).max);
        _mintStETHAndDeposit(supply);
        deal(address(stETH), address(token), supply + surplus);
        token.collectFee();
        assertEq(stETH.balanceOf(address(this)), surplus);
    }

    function testRescue(
        uint256 supply,
        uint256 rebased,
        uint256 withdrawAmount
    ) public {
        supply = bound(supply, 0, type(uint256).max);
        rebased = bound(rebased, 0, supply);
        withdrawAmount = bound(withdrawAmount, 0, supply);
        vm.assume(withdrawAmount != 0);
        vm.assume(rebased != 0);

        // mint supply
        _mintStETHAndDeposit(supply);

        // rebased down, invariant should fail
        deal(address(stETH), address(token), supply - rebased);

        if (withdrawAmount > stETH.balanceOf(address(token))) {
            // unable to transfer enough stETH
            vm.expectRevert();
            token.withdraw(withdrawAmount);
        } else {
            // even though it has enough to transfer stETH, it should trigger invariant check failure
            vm.expectRevert("Invariant check failed");
            token.withdraw(withdrawAmount);
        }

        token.rescueStETH();
        assertEq(stETH.balanceOf(address(this)), supply - rebased);
    }

    function _mintStETHAndDeposit(uint256 amount) internal {
        stETH.mint(address(this), amount);
        stETH.approve(address(token), amount);
        token.deposit(amount);
    }
}
