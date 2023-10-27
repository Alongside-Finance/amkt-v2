pragma solidity =0.8.18;
import {UpgradedTest} from "test/upgrade/helpers/Upgraded.t.sol";
import {fmul, fdiv} from "src/lib/FixedPoint.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {console} from "forge-std/console.sol";
import {IndexToken} from "src/IndexToken.sol";
import {InitialBountyHelper, ETH, AMKT_PROXY, MULTISIG, VOTE_DELAY} from "src/scripts/Config.sol";

interface IOracle {
    function latestAnswer() external returns (uint256);

    function decimals() external returns (uint256);
}

interface IWstETH {
    function getStETHByWstETH(
        uint256 _wstETHAmount
    ) external view returns (uint256);
}

// NOTE TO DEVELOPERS:
// This checklist test is used to ensure safety of the migration process.
// If any one of these tests fail, it means that it is not safe to execute the upgrade bundle.
// These tests should guide the migration process, step by step, to ensure that the upgrade bundle is safe to execute.
contract MigrationChecklistTest is UpgradedTest {
    bool triggerMigrationWarning_getCurrentPrice;

    function test_printBatchExecutionData() public view {
        bool shouldPrintExecutionData = false;
        if (shouldPrintExecutionData) {
            console.logBytes(batchExecutionData);
        }
    }

    // WARNING: Fork block number must be updated prior to simulation
    // Expected date of finalization is October 30, 2023
    function test_MIGRATION_WARNING_forkBlock() public {
        assertFalse(triggerMigrationWarning_forkBlock);
    }

    // WARNING: This test should fail until warping is no longer used in test preparation.
    // Expected date of finalization is October 30, 2023
    function test_MIGRATION_WARNING_warpForward() public {
        assertFalse(triggerMigrationWarning_warpForward);
    }

    // WARNING: This test should fail until safe balances are no longer mocked
    // Expected date of finalization is October 30, 2023
    function test_MIGRATION_WARNING_safeBalances() public {
        assertFalse(triggerMigrationWarning_safeBalances);
    }

    // WARNING: This test should fail until latest prices are known for migration
    // Expected date of finalization is October 30, 2023

    function test_MIGRATION_WARNING_getCurrentPrice() public {
        assertFalse(triggerMigrationWarning_getCurrentPrice);
    }

    // WARNING:
    // This test should fail until new contracts are deployed and addresses are updated.
    // Expected date of finalization is October 30, 2023
    function test_MIGRATION_WARNING_deployedContracts() public {
        assertFalse(triggerMigrationWarning_setDeployedContracts);
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

    // Tests if the deployed contracts are actually contracts
    function test_MIGRATION_WARNING_deployedContractsBehavior() public {
        assertEq(address(vault.indexToken()), AMKT_PROXY);
        assertEq(address(issuance.vault()), address(vault));
        assertEq(address(invokeableBounty.vault()), address(vault));
        assertEq(activeBounty.authority(), MULTISIG);
        assertEq(governor.votingDelay(), VOTE_DELAY);
        assertEq(
            timelockController.TIMELOCK_ADMIN_ROLE(),
            keccak256("TIMELOCK_ADMIN_ROLE")
        );
        assertEq(
            IndexToken(newTokenImplementation).MINTER_SLOT(),
            0x1af730152eea9813c49583a406e8dd55a4df08cae9e33ae45721374fdde82bae
        );
        assertEq(address(timelockInvokeableBounty.vault()), address(vault));
        assertEq(timelockActiveBounty.authority(), address(timelockController));
    }

    // WARNING: This test should fail until `tokens` in InitialBountyHelper is finalized.
    // Expected date of finalization is October 30, 2023
    function test_MIGRATION_WARNING_finalizedTokens() public {
        assertFalse(
            (new InitialBountyHelper()).triggerMigrationWarning_finalTokens()
        );
    }

    // WARNING: This test should fail until `inputBatchExecutionData` is used.
    // Expected date of finalization is October 30, 2023
    function test_MIGRATION_WARNING_executeUpgradeBundle() public {
        assertFalse(triggerMigrationWarning_executeUpgradeBundle);
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
            uint256 adjustedUnits;
            if (tokens[i].token == ETH) {
                adjustedUnits = IWstETH(ETH).getStETHByWstETH(tokens[i].units);
            } else {
                adjustedUnits = tokens[i].units;
            }
            uint256 normalizedUnits = adjustedUnits *
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
        assertGe(nav, 2_500_000e18); // sanity check: greater than 2.5 million
        assertLe(nav, 2_510_000e18); // sanity checl: less than 2.51 million
    }

    // WARNING: Manual prices must be updated upon finalization
    // Expected date of finalization is October 30, 2023
    function MIGRATION_WARNING_getCurrentPrice(
        address token
    ) internal returns (uint256) {
        triggerMigrationWarning_getCurrentPrice = true; // Flip when manual prices are finalized
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
