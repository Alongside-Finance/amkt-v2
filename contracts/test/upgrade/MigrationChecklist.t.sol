pragma solidity =0.8.18;
import {UpgradedTest} from "test/upgrade/helpers/Upgraded.t.sol";
import {fmul, fdiv} from "src/lib/FixedPoint.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";

contract MigrationChecklistTest is UpgradedTest {
    // Native
    address constant BTC = address(0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599);
    address constant ETH = address(0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0);
    address constant MATIC =
        address(0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0);
    address constant FTM = address(0x4E15361FD6b4BB609Fa63C81A2be19d873717870);
    address constant SHIB = address(0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE);
    address constant LINK = address(0x514910771AF9Ca656af840dff83E8264EcF986CA);
    address constant UNI = address(0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984);
    address constant LDO = address(0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32);
    address constant MNT = address(0x3c3a81e81dc49A522A592e7622A7E711c06bf354);
    address constant CRO = address(0xA0b73E1Ff0B80914AB6fe0444E65848C4C34450b);
    address constant QNT = address(0x4a220E6096B25EADb88358cb44068A3248254675);
    address constant ARB = address(0xB50721BCf8d664c30412Cfbc6cf7a15145234ad1);
    address constant MKR = address(0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2);
    address constant AAVE = address(0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9);
    address constant GRT = address(0xc944E90C64B2c07662A292be6244BDf05Cda44a7);

    // Wormhole Bridge
    address constant BNB = address(0x418D75f65a02b3D53B2418FB8E1fe493759c7605);
    address constant SOL = address(0xD31a59c85aE9D8edEFeC411D448f90841571b89c);
    address constant AVAX = address(0x85f138bfEE4ef8e540890CFb48F620571d67Eda3);
    address constant OP = address(0x1df721D242E0783F8fCab4A9FfE4F35bdf329909);

    // Rainbow Bridge
    address constant NEAR = address(0x85F17Cf997934a597031b2E18a9aB6ebD4B9f6a4);

    // WARNING:
    // This test should fail until new contracts are deployed and addresses are updated.
    // Expected date of finalization is October 30, 2023
    function test_MIGRATION_WARNING_deployedContracts() public {
        assertEq(true, false);
        assertEq(address(vault), address(1));
        assertEq(address(issuance), address(1));
        assertEq(address(invokeableBounty), address(1));
        assertEq(address(activeBounty), address(1));
        assertEq(address(governor), address(payable(address(1))));
        assertEq(address(timelockController), address(payable(address(1))));
        assertEq(newTokenImplementation, address(1));
        assertEq(address(timelockActiveBounty), address(1));
        assertEq(address(timelockInvokeableBounty), address(1));
    }

    // WARNING: This test should fail until `inputBatchExecutionData` is known.
    // Expected date of finalization is October 30, 2023
    function test_MIGRATION_WARNING_expectedCalldataMatchesInputCalldata()
        public
    {
        assertEq(batchExecutionData, inputBatchExecutionData);
    }

    // WARNING: This test should fail until final `tokens` in InitialBountyHelper is known.
    // Numbers below must be replaced with current prices at finalization time.
    // Expected date of finalization is October 30, 2023

    function test_MIGRATION_WARNING_navIsCloseEnough() public {
        TokenInfo[] memory prices = new TokenInfo[](15);
        prices[0] = TokenInfo(BTC, 1e22);
        prices[1] = TokenInfo(ETH, 1e21);
        prices[2] = TokenInfo(BNB, 1e20);
        prices[3] = TokenInfo(SOL, 1e19);
        prices[4] = TokenInfo(MATIC, 1e18);
        prices[5] = TokenInfo(SHIB, 1e17);
        prices[6] = TokenInfo(AVAX, 1e16);
        prices[7] = TokenInfo(LINK, 1e15);
        prices[8] = TokenInfo(UNI, 1e14);
        prices[9] = TokenInfo(LDO, 1e13);
        prices[10] = TokenInfo(MNT, 1e12);
        prices[11] = TokenInfo(CRO, 1e11);
        prices[12] = TokenInfo(QNT, 1e10);
        prices[13] = TokenInfo(ARB, 1e9);
        prices[14] = TokenInfo(MKR, 1e8);
        TokenInfo[] memory units = vault.virtualUnits();
        uint256 nav;
        for (uint256 i = 0; i < units.length; i++) {
            assertEq(units[i].token, prices[i].token);
            nav += fdiv(
                fmul(prices[i].units, units[i].units),
                10 ** (IERC20(units[i].token).decimals())
            );
        }
        assertGe(nav, 2_400_000e18); // greater than 2.4 million
        assertLe(nav, 2_600_000e18); // less than 2.6 million
    }
}
