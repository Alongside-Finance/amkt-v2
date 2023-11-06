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
        bool shouldPrintExecutionData = true;
        if (shouldPrintExecutionData) {
            console.logBytes(batchExecutionData);
        }
    }

    // WARNING: Fork block number must be updated prior to simulation
    // Expected date of finalization is October 30, 2023
    function test_MIGRATION_WARNING_forkBlock() public {
        assertFalse(triggerMigrationWarning_forkBlock);
    }

    // WARNING: This test should fail until safe balances are no longer mocked
    // Expected date of finalization is October 30, 2023
    function test_MIGRATION_WARNING_safeBalances() public {
        assertFalse(triggerMigrationWarning_safeBalances);
    }

    // WARNING:
    // This test should fail until new contracts are deployed and addresses are updated.
    // Expected date of finalization is October 30, 2023
    function test_MIGRATION_WARNING_deployedContracts() public {
        assertFalse(triggerMigrationWarning_setDeployedContracts);
        assertEq(
            address(vault),
            address(0xf3bCeDaB2998933c6AAD1cB31430D8bAb329dD8C)
        );
        assertEq(
            address(issuance),
            address(0x7D1775061A3a713E778aF23806330B532Fa006B0)
        );
        assertEq(
            address(invokeableBounty),
            address(0xE13Ee59C41c67696754277cDC73710f6D65Ef2Ac)
        );
        assertEq(
            address(activeBounty),
            address(0x0DAF7e851f6054085432229150c1706988aBc562)
        );
        assertEq(
            address(governor),
            address(
                payable(address(0xb6a6f2a56693Dc4f893f8396D945f7dFe03aA9ba))
            )
        );
        assertEq(
            address(timelockController),
            address(
                payable(address(0x4c362faB50Bc81F0F58ef2DA6b6E10b55FC1d478))
            )
        );
        assertEq(
            newTokenImplementation,
            address(0xE006fC6Dc996Ba5361Cb880fEf76f8aA9D5A7f70)
        );
        assertEq(
            address(quoter),
            address(0xE3BE63E1B959c152212ce1dD45D0d2f749eB227c)
        );
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
        if (
            keccak256(abi.encodePacked(forkOverrideUrl)) ==
            keccak256(abi.encodePacked("none"))
        ) {
            assertEq(batchExecutionData, inputBatchExecutionData);
        }
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
        // 3078134.67677
        assertGe(nav, 3_000_000e18); // sanity check: greater than 3 million TODO: pull this number from market cap
        assertLe(nav, 3_100_000e18); // sanity check: less than 3.1 million TODO: pull this number from market cap
        assertFalse(triggerMigrationWarning_getCurrentPrice); // This test should fail until latest prices are known for migration
    }

    // WARNING: Manual prices must be updated upon finalization
    // Expected date of finalization is October 30, 2023
    function MIGRATION_WARNING_getCurrentPrice(
        address token
    ) internal returns (uint256) {
        triggerMigrationWarning_getCurrentPrice = false; // Flip when manual prices are finalized
        address[] memory oracles = new address[](15);
        uint256[] memory manualPrices = new uint256[](15); // for tokens that don't have a chainlink oracle, use prices manually
        oracles[0] = address(0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c); // BTC
        oracles[1] = address(0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419); // ETH
        oracles[2] = address(0x14e613AC84a31f709eadbdF89C6CC390fDc9540A); // BNB
        oracles[3] = address(0x4ffC43a60e009B551865A93d232E33Fce9f01507); // SOL
        oracles[4] = address(0x7bAC85A8a13A4BcD8abb3eB7d6b4d632c5a57676); // MATIC
        oracles[5] = address(0x2c1d072e956AFFC0D435Cb7AC38EF18d24d9127c); // LINK
        manualPrices[6] = 7790000000000; // SHIB
        oracles[7] = address(0xFF3EEb22B5E3dE6e705b44749C2559d704923FD7); // AVAX
        oracles[8] = address(0x553303d460EE0afB37EdFf9bE42922D8FF63220e); // UNI
        oracles[9] = address(0xec1D1B3b0443256cc3860e24a46F108e699484Aa); // MKR
        manualPrices[10] = 1770000000000000000; //LDO
        manualPrices[11] = 60563000000000000; // CRO
        manualPrices[12] = 375308000000000000; // MNT
        manualPrices[13] = 1380000000000000000; // OP
        manualPrices[14] = 102200000000000000000; // QNT

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
