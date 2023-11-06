import {astETH} from "periphery/astETH.sol";
import {BaseTest} from "test/utils/BaseTest.t.sol";
import {MockMintableToken} from "test/utils/MockMintableToken.sol";

// 0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84 stETH on mainnet

contract astETHTest is BaseTest {
    astETH token;
    MockMintableToken sourceToken;

    function setUp() public {
        sourceToken = new MockMintableToken("stETH", "stETH", 18, 100e18);
        token = new astETH(address(sourceToken), address(this));
    }

    function testDeposit() public {
        uint256 amount = 100e18;
        sourceToken.approve(address(token), amount);
        token.deposit(amount);
        assertEq(token.balanceOf(address(this)), amount);
    }

    function testWithdraw() public {
        uint256 amount = 100e18;
        sourceToken.approve(address(token), amount);
        token.deposit(amount);
        token.withdraw(amount);
        assertEq(token.balanceOf(address(this)), 0);
    }

    function testCollectFee() public {
        uint256 amount = 100e18;
        sourceToken.approve(address(token), amount);
        token.deposit(amount);
        deal(address(sourceToken), address(token), 101e18);
        token.collectFee();
        assertEq(sourceToken.balanceOf(address(this)), 1e18);
    }
}
