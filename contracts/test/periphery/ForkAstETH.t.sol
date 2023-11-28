import {AstETH, IStETH} from "periphery/AstETH.sol";
import {BaseTest} from "test/utils/BaseTest.t.sol";
import {MockMintableToken} from "test/utils/MockMintableToken.sol";

contract AstForkETHTest is BaseTest {
    AstETH token;
    IStETH stETH;

    function setUp() public {
        vm.createSelectFork(
            "https://mainnet.infura.io/v3/2c9945ed9e3c48bd8a3c7166ddd45057",
            18608317
        );
        stETH = IStETH(0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84);
        token = new AstETH(
            IStETH(address(stETH)),
            address(this),
            address(this)
        );
    }

    function testWrapAndDeposit(uint256 amount) public {
        uint256 STAKE_LIMIT = 1.5e23;
        amount = bound(amount, 0, STAKE_LIMIT); // cannot deposit more than stake limit
        vm.assume(amount != 0); // cannot deposit 0
        vm.deal(address(this), amount);
        token.wrapAndDeposit{value: amount}();
        assertGt(amount, token.balanceOf(address(this)));
        assertLt(amount, token.balanceOf(address(this)) + 3);
    }
}
