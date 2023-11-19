import {AstETH, IStETH} from "periphery/AstETH.sol";
import {BaseTest} from "test/utils/BaseTest.t.sol";
import {MockMintableToken} from "test/utils/MockMintableToken.sol";

// 0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84 stETH on mainnet

contract astETHTest is BaseTest {
    AstETH token;
    MockMintableToken stETH;

    function setUp() public {
        stETH = new MockMintableToken("stETH", "stETH", 18, 0);
        token = new AstETH(
            IStETH(address(stETH)),
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

    function testSlashing(
        uint256 supply,
        uint256 rebased,
        uint256 withdrawAmount
    ) public {
        supply = bound(supply, 0, type(uint16).max);
        rebased = bound(rebased, 0, supply);
        withdrawAmount = bound(withdrawAmount, 0, supply);
        vm.assume(withdrawAmount != 0);
        vm.assume(rebased != 0);

        // mint supply
        _mintStETHAndDeposit(supply);

        // rebased down, amount received should be proportional
        deal(address(stETH), address(token), supply - rebased);

        token.withdraw(withdrawAmount);
        assertEq(
            stETH.balanceOf(address(this)),
            (withdrawAmount * (supply - rebased)) / supply
        );
    }

    function _mintStETHAndDeposit(uint256 amount) internal {
        stETH.mint(address(this), amount);
        stETH.approve(address(token), amount);
        token.deposit(amount);
    }
}
