import {AstETH, IStETH} from "periphery/AstETH.sol";
import {BaseTest} from "test/utils/BaseTest.t.sol";
import {MockMintableToken} from "test/utils/MockMintableToken.sol";
import {console2} from "forge-std/console2.sol";

contract AstETHTest is BaseTest {
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
        _mintStETHAndDeposit(amount, address(this));
        assertEq(token.balanceOf(address(this)), amount);
    }

    function testWithdraw(uint256 amount) public {
        _mintStETHAndDeposit(amount, address(this));
        token.withdraw(amount);
        assertEq(token.balanceOf(address(this)), 0);
    }

    function testCollectFee(uint256 supply, uint256 surplus) public {
        supply = bound(supply, 0, type(uint128).max);
        surplus = bound(surplus, 0, type(uint128).max);
        _mintStETHAndDeposit(supply, address(this));
        _rebaseUp(surplus);
        token.collectFee();
        assertEq(stETH.balanceOf(address(this)), surplus);
    }

    function testSlashingWithdrawalProrated(
        uint256 supply,
        uint256 slashed,
        uint256 withdrawAmount
    ) public {
        supply = bound(supply, 0, type(uint128).max);
        slashed = bound(slashed, 0, supply);
        withdrawAmount = bound(withdrawAmount, 0, supply);
        vm.assume(withdrawAmount != 0);
        vm.assume(slashed != 0);

        // mint supply
        _mintStETHAndDeposit(supply, address(this));

        // rebased down, amount received should be proportional
        _rebaseDown(slashed);

        token.withdraw(withdrawAmount);
        assertEq(
            stETH.balanceOf(address(this)),
            (withdrawAmount * (supply - slashed)) / supply
        );
    }

    // TODO: Fuzz this
    function testAttackWithKnownSlashing() public {
        _mintStETHAndDeposit(300e18, address(this));
        address attacker = address(2);
        // 1% rebase down is known
        //take out a flashloan
        uint256 attackerLoanAmount = 29700e18; // borrows 29700 wstETH, where 1 wstETH = 1 stETH
        assertEq(stETH.balanceOf(attacker), 0);
        _mintStETHAndDeposit(29700e18, attacker);
        assertEq(token.totalSupply(), 30000e18);
        assertEq(stETH.balanceOf(address(token)), 30000e18);
        // trigger 1% rebase down
        _rebaseDown(300e18); // 1 wstETH = ~0.99 stETH
        assertEq(token.totalSupply(), 30000e18);
        assertEq(stETH.balanceOf(address(token)), 29700e18);
        vm.prank(attacker);
        token.withdraw(29700e18);
        assertEq(stETH.balanceOf(attacker), 29403e18); // 29403 = 29700 * 0.99
        uint256 revenue = (stETH.balanceOf(attacker) * 100e18) / 99e18;
        uint256 profit = revenue - attackerLoanAmount;
        assertEq(profit, 0);
    }

    // TODO: What happens if stETH rebases down, but there are still fees yet to be collected?
    function testAttackWithKnownSlashingLittleProfit() public {
        _mintStETHAndDeposit(300e18, address(this));
        address attacker = address(2);
        // 1% rebase down is known
        //take out a flashloan
        uint256 attackerLoanAmount = 29700e18; // borrows 29700 wstETH, where 1 wstETH = 1 stETH
        assertEq(stETH.balanceOf(attacker), 0);
        _mintStETHAndDeposit(29700e18, attacker);
        assertEq(token.totalSupply(), 30000e18);
        assertEq(stETH.balanceOf(address(token)), 30000e18);
        // simulate fees
        _rebaseUp(270e18);
        // trigger 1% rebase down
        _rebaseDown(300e18); // 1 wstETH = ~0.99 stETH

        assertEq(token.totalSupply(), 30000e18);
        assertEq(stETH.balanceOf(address(token)), 29970e18);
        vm.prank(attacker);
        token.withdraw(29700e18);
        // assertEq(stETH.balanceOf(attacker), 29403e18); // 29403 = 29700 * 0.99
        uint256 revenue = (stETH.balanceOf(attacker) * 100e18) / 99e18;
        uint256 profit = revenue - attackerLoanAmount;
        assertEq(profit, 270e18);
    }

    function _mintStETHAndDeposit(uint256 amount, address user) internal {
        stETH.mint(user, amount);
        vm.startPrank(user);
        stETH.approve(address(token), amount);
        token.deposit(amount);
        vm.stopPrank();
    }

    function _rebaseUp(uint256 amount) internal {
        deal(
            address(stETH),
            address(token),
            stETH.balanceOf(address(token)) + amount
        );
    }

    function _rebaseDown(uint256 amount) internal {
        deal(
            address(stETH),
            address(token),
            stETH.balanceOf(address(token)) - amount
        );
    }
}
