pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {StatefulTest} from "./State.t.sol";
import {IVault} from "src/interfaces/IVault.sol";
import {TokenInfo} from "src/Common.sol";
import {fmul, fdiv} from "src/lib/FixedPoint.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";

contract VaultTest is StatefulTest {
    function testZeroChecks() public {
        vm.expectRevert(IVault.VaultZeroCheck.selector);
        vault.setIssuance(address(0));

        vm.expectRevert(IVault.VaultZeroCheck.selector);
        vault.setRebalancer(address(0));

        vm.expectRevert(IVault.VaultZeroCheck.selector);
        vault.setFeeRecipient(address(0));

        vm.expectRevert(IVault.VaultZeroCheck.selector);
        vault.setEmergencyResponder(address(0));
    }

    function testVaultFeeTooSmall() public {
        seedInitial(10);
        vault.setInflationRate(0);
        _warpForward(2 days);
        vm.startPrank(feeReciever);
        vm.expectRevert(IVault.AMKTVaultFeeTooSmall.selector);
        vault.tryInflation();
        vm.stopPrank();
    }

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
        vm.expectRevert(IVault.AMKTVaultInflationRateTooLarge.selector);
        vault.setInflationRate(1e18 + 1); // SCALAR is 1e18
        vault.setInflationRate(1e18);
    }

    function emergencyResponderFunctions(bool toFail) public {
        if (toFail) vm.expectRevert();
        vault.setEmergency(true);
        if (toFail) vm.expectRevert();
        vault.setEmergency(false);
    }

    function rebalancerFunctions(bool toFail) public {
        if (toFail) vm.expectRevert();
        IVault.SetNominalArgs[] memory args = new IVault.SetNominalArgs[](1);
        args[0] = IVault.SetNominalArgs(address(0), 1);
        vault.invokeSetNominals(args);
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
        IVault.InvokeERC20Args[] memory args = new IVault.InvokeERC20Args[](1);
        args[0] = IVault.InvokeERC20Args(tokens[0], address(0), 0);
        vault.invokeERC20s(args);
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
        vault.setInflationRate(0);
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
        inflationTestHelper(1000, 26277028992000000);
    }

    function testInflation730() public {
        inflationTestHelper(730, 19182231164160000);
    }

    function testInflation365() public {
        inflationTestHelper(365, 9591115582080000);
    }

    function testInflation30() public {
        inflationTestHelper(30, 788310869760000);
    }

    function testInflation7() public {
        inflationTestHelper(7, 183939202944000);
    }

    function testInflation1() public {
        inflationTestHelper(1, 26277028992000);
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
        assertEq(newSupply, initialSupply + expectedInflation);
        assertEq(
            newFeeRecipientBalance,
            initialFeeRecipientBalance + expectedInflation
        );
        uint256 valueMultiplier = fdiv(
            initialSupply,
            initialSupply + expectedInflation
        );
        TokenInfo[] memory newUnits = vault.virtualUnits();
        for (uint256 i = 0; i < newUnits.length; i++) {
            assertEq(
                newUnits[i].units,
                fmul(initialUnits[i].units, valueMultiplier)
            );
        }
    }
}
