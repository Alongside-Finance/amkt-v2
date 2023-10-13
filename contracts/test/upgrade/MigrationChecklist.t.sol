pragma solidity =0.8.18;
import {UpgradedTest} from "test/upgrade/helpers/Upgraded.t.sol";
import {fmul, fdiv} from "src/lib/FixedPoint.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {console} from "forge-std/console.sol";

interface IOracle {
    function latestAnswer() external returns (uint256);

    function decimals() external returns (uint256);
}

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
        uint256 totalSupply = AMKT.totalSupply();
        TokenInfo[] memory tokens = vault.virtualUnits();
        uint256 nav;
        for (uint256 i = 0; i < tokens.length; i++) {
            uint256 normalizedUnits = tokens[i].units *
                (10 ** (18 - IERC20(tokens[i].token).decimals()));
            uint256 value = fmul(
                totalSupply,
                fmul(
                    MIGRATION_WARNING_getCurrentPrice(tokens[i].token),
                    normalizedUnits
                )
            );
            uint256 humanReadableValue = value / (10 ** 18);
            console.log("value: %s", humanReadableValue);
            nav += value;
        }
        // WARNING: update expected NAV value here
        assertGe(nav, 2_400_000e18); // sanity check: greater than 2.4 million
        assertLe(nav, 2_600_000e18); // sanity checl: less than 2.6 million
    }

    // WARNING: Manual prices must be updated upon finalization
    // Expected date of finalization is October 30, 2023
    function MIGRATION_WARNING_getCurrentPrice(
        address token
    ) internal returns (uint256) {
        address[] memory oracles = new address[](15);
        uint256[] memory manualPrices = new uint256[](15); // for tokens that don't have a chainlink oracle, use prices manually
        oracles[0] = address(0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c); // BTC
        oracles[1] = address(0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419); // ETH
        oracles[2] = address(0x14e613AC84a31f709eadbdF89C6CC390fDc9540A); // BNB
        oracles[3] = address(0x4ffC43a60e009B551865A93d232E33Fce9f01507); // SOL
        oracles[4] = address(0x7bAC85A8a13A4BcD8abb3eB7d6b4d632c5a57676); // MATIC
        oracles[5] = address(0x2c1d072e956AFFC0D435Cb7AC38EF18d24d9127c); // LINK
        manualPrices[6] = 6910000000000; // SHIB
        oracles[7] = address(0xFF3EEb22B5E3dE6e705b44749C2559d704923FD7); // AVAX
        oracles[8] = address(0x553303d460EE0afB37EdFf9bE42922D8FF63220e); // UNI
        oracles[9] = address(0xec1D1B3b0443256cc3860e24a46F108e699484Aa); // MKR
        manualPrices[10] = 15200000000000000; //LDO
        manualPrices[11] = 490430000000000; // CRO
        manualPrices[12] = 3528950000000000; // MNT
        manualPrices[13] = 12400000000000000; // OP
        manualPrices[14] = 85770000000000000000; // QNT

        TokenInfo[] memory tokens = vault.virtualUnits();
        for (uint256 i = 0; i < tokens.length; i++) {
            if (tokens[i].token == token) {
                if (oracles[i] != address(0)) {
                    uint256 answer = IOracle(oracles[i]).latestAnswer();
                    uint256 decimals = IOracle(oracles[i]).decimals();
                    uint256 normalized = answer * (10 ** (18 - decimals));
                    uint256 humanReadable = normalized / (10 ** 18);
                    console.log(
                        "token: %s, price: %s",
                        tokens[i].token,
                        humanReadable
                    );
                    return normalized;
                } else {
                    return manualPrices[i];
                }
            }
        }
        return 0;
    }
}
