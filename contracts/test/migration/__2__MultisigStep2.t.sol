pragma solidity =0.8.15;

import "forge-std/Test.sol";
import {__1__MultisigStep1Test} from "./__1__MultisigStep1.t.sol";
import {MultisigStep2Script} from "src/scripts/__2__MultisigStep2.s.sol";

contract __2__MultisigStep2Test is __1__MultisigStep1Test {
    function setUp() public virtual override {
        super.setUp();
        MultisigStep2Script script = new MultisigStep2Script();
        script.run(
            address(AMKT),
            newTokenImplementation,
            address(vault),
            address(timelockController),
            address(timelockInvokeableBounty)
        );
    }
}
