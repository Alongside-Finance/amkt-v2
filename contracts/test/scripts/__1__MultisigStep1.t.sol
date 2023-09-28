pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {__0__CoreDeployTest} from "./__0__CoreDeploy.t.sol";
import {MultisigStep1Script} from "src/scripts/__1__MultisigStep1.s.sol";
import {InitialBountyHelper, MULTISIG} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {Dealer} from "test/Dealer.t.sol";

contract __1__MultisigStep1Test is __0__CoreDeployTest {
    function setUp() public virtual override {
        super.setUp();
        mockBalancesAndApprovals();
        MultisigStep1Script script = new MultisigStep1Script();
        script.run(
            address(vault),
            address(invokeableBounty),
            address(activeBounty)
        );
    }

    function mockBalancesAndApprovals() internal {
        Dealer dealer = new Dealer();
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();
        for (uint256 i = 0; i < tokens.length; i++) {
            dealer.dealToken(tokens[i].token, MULTISIG, AMKT.totalSupply());
        }
        vm.startPrank(MULTISIG);
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            token.approve(address(invokeableBounty), token.balanceOf(MULTISIG));
            token.approve(address(issuance), token.balanceOf(MULTISIG));
        }

        vm.stopPrank();
    }
}
