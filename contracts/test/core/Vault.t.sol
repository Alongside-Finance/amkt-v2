pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {StatefulTest} from "./State.t.sol";
import {IVault} from "src/interfaces/IVault.sol";
import {TokenInfo} from "src/Common.sol";
import {fmul, fdiv} from "src/lib/FixedPoint.sol";

contract VaultTest is StatefulTest {
    function testShouldAllowRebalancer() public {
        seedInitial(10);
        vm.startPrank(address(bounty));
        rebalancerFunctions({toFail: false});
    }

    function testShouldAllowIssuance() public {
        seedInitial(10);
        vm.startPrank(address(issuance));
        issuanceFunctions({toFail: false});
    }

    function testShouldAllowEither() public {
        seedInitial(10);
        vm.startPrank(address(bounty));
        eitherFunctions({toFail: false});

        vm.stopPrank();

        vm.startPrank(address(issuance));
        eitherFunctions({toFail: false});
    }

    function testShouldAllowOwner() public {
        ownerFunctions({toFail: false});
    }

    function shouldAllowEmergencyResponder() public {
        vm.startPrank(emergencyResponder);
        emergencyResponderFunctions(false);
        vm.stopPrank();
    }

    function testShouldNotAllowNonRebalancer() public {
        seedInitial(10);
        rebalancerFunctions(true);
    }

    function testShouldNotAllowNonIssuance() public {
        seedInitial(10);
        issuanceFunctions(true);
    }

    function testShouldNotAllowNonEither() public {
        seedInitial(10);
        eitherFunctions(true);
    }

    function testShouldNotAllowNonEmergencyResponder() public {
        seedInitial(10);
        emergencyResponderFunctions(true);
    }

    function testShouldNotAllowNonOwner(address addr) public {
        vm.assume(addr != address(this));
        vm.startPrank(addr);
        ownerFunctions(true);
    }

    function testShouldNotAllowSetFeeTooLarge() public {
        vm.expectRevert();
        vault.setFee(1e18 + 1); // SCALAR is 1e18
        vault.setFee(1e18);
    }

    function emergencyResponderFunctions(bool toFail) public {
        if (toFail) vm.expectRevert();
        vault.setEmergency(true);
        if (toFail) vm.expectRevert();
        vault.setEmergency(false);
    }

    function rebalancerFunctions(bool toFail) public {
        if (toFail) vm.expectRevert();
        vault.invokeSetNominal(IVault.SetNominalArgs(address(0), 1));
        // TODO: set nominals
    }

    function issuanceFunctions(bool toFail) public {
        if (toFail) vm.expectRevert();
        vault.invokeMint(address(0), 0);
        if (toFail) vm.expectRevert();
        vault.invokeBurn(address(0), 0);
    }

    function eitherFunctions(bool toFail) public {
        address[] memory tokens = vault.underlying();

        if (toFail) vm.expectRevert();
        vault.invokeERC20(IVault.InvokeERC20Args(tokens[0], address(0), 0));
    }

    function ownerFunctions(bool toFail) public {
        if (toFail) vm.expectRevert();
        vault.transferOwnership(address(this));
        if (toFail) vm.expectRevert();
        vault.setIssuance(address(1));
        if (toFail) vm.expectRevert();
        vault.setRebalancer(address(1));
        if (toFail) vm.expectRevert();
        vault.setFeeRecipient(address(1));
        if (toFail) vm.expectRevert();
        vault.setFee(0);
        if (toFail) vm.expectRevert();
        vault.setEmergencyResponder(address(1));
    }

    function testVirtualUnits() public {
        TokenInfo[] memory tokens = seedInitial(10);

        TokenInfo[] memory virtualUnitsInfo = vault.virtualUnits();
        for (uint256 i = 0; i < virtualUnitsInfo.length; i++) {
            TokenInfo memory info = virtualUnitsInfo[i];
            uint256 virtualUnits = info.units;

            assertEq(
                virtualUnits,
                tokens[i].units,
                "Virtual units should match the expected value"
            );
        }
    }

    function testZeroInflation() public {
        vm.startPrank(feeReciever);
        assertEq(indexToken.totalSupply(), 1e18);
        vm.expectRevert(IVault.AMKTVaultFeeTooEarly.selector);
        vault.tryInflation();
    }

    function testInflation1000() public {
        inflationTestHelper(1000, 26722925457102672);
    }

    function testInflation730() public {
        inflationTestHelper(730, 19367991845056065);
    }

    function testInflation365() public {
        inflationTestHelper(365, 9591115598182735);
    }

    function testInflation30() public {
        inflationTestHelper(30, 781432077101298);
    }

    function testInflation7() public {
        inflationTestHelper(7, 182224980715664);
    }

    function testInflation1() public {
        inflationTestHelper(1, 26028074703314);
    }

    function inflationTestHelper(
        uint256 daysPassed,
        uint256 expectedInflation
    ) internal {
        seedInitial(10);
        uint256 initialSupply = indexToken.totalSupply();
        uint256 initialFeeRecipientBalance = indexToken.balanceOf(feeReciever);
        TokenInfo[] memory initialUnits = vault.virtualUnits();
        vm.warp(block.timestamp + 1 days * daysPassed);
        vm.prank(feeReciever);
        vault.tryInflation();
        uint256 newSupply = indexToken.totalSupply();
        uint256 newFeeRecipientBalance = indexToken.balanceOf(feeReciever);
        requireCloseX10(newSupply, initialSupply + expectedInflation);
        requireCloseX5(newFeeRecipientBalance, expectedInflation); // todo increase supply?

        // recover the decay
        // initialSupply + expectedInflation = initialSupply * (1 / decay)
        //
        // =>  initialSupply / (initialSupply + expectedInflation)
        //     = initialSupply / newSupply
        //     = 1 / 1 / d = d
        uint256 decay = fdiv(initialSupply, initialSupply + expectedInflation);

        TokenInfo[] memory newUnits = vault.virtualUnits();
        for (uint256 i = 0; i < newUnits.length; i++) {
            requireCloseX10(
                newUnits[i].units,
                fmul(initialUnits[i].units, decay)
            );
        }
    }

    // the values are a facotr of 1/10_000_000_000 apart
    function requireCloseX10(uint256 a, uint256 b) internal {
        rangeCheck(a, b, 1, 1e10);
    }

    // the values are a factor of 1/100_000 apart
    // needed for smaller numbers esp casue less digits means less allowable error
    function requireCloseX5(uint256 a, uint256 b) internal {
        rangeCheck(a, b, 1, 1e5);
    }
}
