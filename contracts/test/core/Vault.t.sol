pragma solidity =0.8.15;

import "forge-std/Test.sol";
import {StatefulTest} from "./State.t.sol";
import {IVault} from "src/interfaces/IVault.sol";
import {TokenInfo} from "src/Common.sol";
import {FEE_SCALED} from "src/scripts/Config.sol";

contract VaultTest is StatefulTest {
    function testIntradayMultiplier() public {
        seedInitial(10);
        vm.warp(block.timestamp + 1 days);
        vault.intradayMultiplier();
    }

    function testFuzzIntradayMultiplier(uint256 targetTimestamp) public {
        vm.assume(
            targetTimestamp >= block.timestamp &&
                targetTimestamp < block.timestamp + 2 days
        );
        seedInitial(10);
        vm.warp(targetTimestamp);
        vault.intradayMultiplier();
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
        vm.expectRevert();
        vault.setFeeScaled(1e18 + 1); // SCALAR is 1e18
        vault.setFeeScaled(1e18);
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
        if (toFail) vm.expectRevert();
        vault.invokeSetMultiplier(0);
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
        vault.setIssuance(address(0));
        if (toFail) vm.expectRevert();
        vault.setRebalancer(address(0));
        if (toFail) vm.expectRevert();
        vault.setFeeRecipient(address(0));
        if (toFail) vm.expectRevert();
        vault.setFeeScaled(0);
        if (toFail) vm.expectRevert();
        vault.setEmergencyResponder(address(0));
    }

    function testRealUnits() public {
        TokenInfo[] memory tokens = seedInitial(10);

        address[] memory underlyingTokens = vault.underlying();
        for (uint256 i = 0; i < underlyingTokens.length; i++) {
            address token = underlyingTokens[i];
            uint256 realUnits = vault.realUnits(token);

            assertEq(
                realUnits,
                tokens[i].units,
                "Real units should match the expected value"
            );
        }
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

    function testInflation() public {
        seedInitial(10);

        uint256 initialSupply = indexToken.totalSupply();
        uint256 initialFeeRecipientBalance = indexToken.balanceOf(feeReciever);
        vm.warp(block.timestamp + 1 days * 365);

        uint256 newMultiplier = vault.tryInflation();

        uint256 newSupply = indexToken.totalSupply();
        uint256 newFeeRecipientBalance = indexToken.balanceOf(feeReciever);

        // Calculate the expected inflation and fee recipient balance

        (, , , uint256 currentMultiplier) = vault.multiplier();

        // Check that the new multiplier is updated correctly
        assertEq(newMultiplier, currentMultiplier);

        // Check that the total supply has increased by the expected inflation
        rangeCheck({
            target: 1009591115598182735, //(1 - (95 / 10000))^(-1) * startingBalance = expectedTotalSupply
            actual: newSupply,
            rangeNumerator: 1,
            rangeDenominator: 1e16
        });

        // // Check that the fee recipient's balance has increased by the expected inflation

        rangeCheck({
            target: 9591115598182735,
            actual: newFeeRecipientBalance,
            rangeNumerator: 1,
            rangeDenominator: 1e12
        });
    }

    function testZeroInflation() public {
        assertEq(indexToken.totalSupply(), 1e18);
        vault.tryInflation();
        assertEq(indexToken.totalSupply(), 1e18);
        vm.warp(block.timestamp + 1 days - 30);
        vault.tryInflation();
        assertEq(indexToken.totalSupply(), 1e18);
    }
}
