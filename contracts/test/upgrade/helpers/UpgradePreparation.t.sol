pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {GnosisTest, GnosisTransaction} from "./Gnosis.t.sol";
import {CoreDeployScript} from "src/scripts/CoreDeploy.s.sol";
import {InitialBountyHelper, AMKT_PROXY, MULTISIG, PROXY_ADMIN, INFLATION_RATE, CANCELLATION_PERIOD, AVG_BLOCK_TIME} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";
import {InvokeableBounty} from "src/invoke/Bounty.sol";
import {Bounty} from "src/interfaces/IInvokeableBounty.sol";
import {ActiveBounty} from "src/invoke/ActiveBounty.sol";
import {Dealer} from "test/utils/Dealer.t.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {IndexToken} from "src/IndexToken.sol";
import {Vault} from "src/Vault.sol";
import {AlongsideGovernor} from "src/Governor.sol";
import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";
import {Issuance} from "src/invoke/Issuance.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {Quoter} from "periphery/Quoter.sol";

contract UpgradePreparationTest is GnosisTest {
    IndexToken AMKT;
    Vault vault;
    Issuance issuance;
    InvokeableBounty invokeableBounty;
    ActiveBounty activeBounty;
    AlongsideGovernor governor;
    TimelockController timelockController;
    address newTokenImplementation;
    Quoter quoter;
    bytes batchExecutionData;

    bool triggerMigrationWarning_forkBlock;
    bool triggerMigrationWarning_setDeployedContracts;
    bool triggerMigrationWarning_safeBalances;

    constructor() GnosisTest(MULTISIG) {}

    function setUp() public virtual {
        fork();
        enableSimulation();
        setDeployedContracts();
        GnosisTransaction[] memory batch = createUpgradeBatch();
        batchExecutionData = getBatchExecutionData(batch);

        // sanity checks
        checkSafeBalances();
        checkCoreDeploy();
    }

    // WARNING: Fork block number must be updated prior to simulation
    // Expected date of finalization is October 30, 2023
    function fork() internal {
        vm.createSelectFork(vm.envString("MAINNET_RPC"));
    }

    function setDeployedContracts() internal {
        triggerMigrationWarning_setDeployedContracts = false;

        AMKT = IndexToken(AMKT_PROXY);
        vault = Vault(address(0xf3bCeDaB2998933c6AAD1cB31430D8bAb329dD8C));
        issuance = Issuance(
            address(0x7D1775061A3a713E778aF23806330B532Fa006B0)
        );
        invokeableBounty = InvokeableBounty(
            address(0xE13Ee59C41c67696754277cDC73710f6D65Ef2Ac)
        );
        activeBounty = ActiveBounty(
            address(0x0DAF7e851f6054085432229150c1706988aBc562)
        );
        governor = AlongsideGovernor(
            payable(address(0xb6a6f2a56693Dc4f893f8396D945f7dFe03aA9ba))
        );
        timelockController = TimelockController(
            payable(address(0x4c362faB50Bc81F0F58ef2DA6b6E10b55FC1d478))
        );
        newTokenImplementation = address(
            0xE006fC6Dc996Ba5361Cb880fEf76f8aA9D5A7f70
        );
        quoter = Quoter(address(0xE3BE63E1B959c152212ce1dD45D0d2f749eB227c));
    }

    // BATCH DESCRIPTION
    // 0-14: Approve tokens to invokeable bounty
    // 15: Set bounty hash to active bounty
    // 16: Accept ownership of vault
    // 17: Fulfill initial bounty
    // 18: Upgrade and initialize
    // 19: Set fee scaled
    // 20: Set rebalancer to timeblock bounty
    // 21: Transfer vault ownership to timelock
    // 22: Transfer proxyAdmin ownership to timelock
    function createUpgradeBatch() public returns (GnosisTransaction[] memory) {
        // Initialize batch with known size
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();
        uint256 batchLength = tokens.length + 8;
        GnosisTransaction[] memory batch = new GnosisTransaction[](batchLength);

        // Approve tokens to invokeable bounty
        uint256 totalSupply = AMKT.totalSupply();
        for (uint256 i = 0; i < 15; i++) {
            batch[i] = GnosisTransaction({
                to: tokens[i].token,
                data: abi.encodeWithSelector(
                    bytes4(keccak256("approve(address,uint256)")),
                    invokeableBounty,
                    fmul(tokens[i].units + 1, totalSupply) + 1
                )
            });
        }

        // Set hash to ActiveBounty for initial bounty
        Bounty memory _bountyToSet = Bounty({
            infos: tokens,
            fulfiller: MULTISIG,
            salt: keccak256(abi.encode("AMKT v2 initial bounty")),
            deadline: 1699315200 // November 7, 2023 0:0:0 GMT
        });

        bytes32 hashToSet = InvokeableBounty(invokeableBounty).hashBounty(
            _bountyToSet
        );

        batch[15] = GnosisTransaction({
            to: address(activeBounty),
            data: abi.encodeWithSelector(
                bytes4(keccak256("setHash(bytes32)")),
                hashToSet
            )
        });

        // Accept ownership of Vault
        batch[16] = GnosisTransaction({
            to: address(vault),
            data: abi.encodeWithSelector(bytes4(keccak256("acceptOwnership()")))
        });

        // Fulfill initial bounty
        batch[17] = GnosisTransaction({
            to: address(invokeableBounty),
            data: abi.encodeWithSelector(
                bytes4(
                    keccak256(
                        "fulfillBounty(((address,uint256)[],address,uint256,bytes32),bool)"
                    )
                ),
                _bountyToSet,
                false
            )
        });

        // Upgrade and initialize
        batch[18] = GnosisTransaction({
            to: PROXY_ADMIN,
            data: abi.encodeWithSelector(
                bytes4(keccak256("upgradeAndCall(address,address,bytes)")),
                AMKT_PROXY,
                newTokenImplementation,
                abi.encodeWithSignature("initialize(address)", vault)
            )
        });

        // Set fee scaled
        batch[19] = GnosisTransaction({
            to: address(vault),
            data: abi.encodeWithSelector(
                bytes4(keccak256("setInflationRate(uint256)")),
                INFLATION_RATE
            )
        });

        // Transfer vault ownership to timelock
        batch[20] = GnosisTransaction({
            to: address(vault),
            data: abi.encodeWithSelector(
                bytes4(keccak256("transferOwnership(address)")),
                timelockController
            )
        });

        // Transfer proxyAdmin ownership to timelock
        batch[21] = GnosisTransaction({
            to: PROXY_ADMIN,
            data: abi.encodeWithSelector(
                bytes4(keccak256("transferOwnership(address)")),
                timelockController
            )
        });
        return batch;
    }

    // check that the safe balance matches exactly what the initial bounty helper expects
    function checkSafeBalances() public {
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();
        uint256 totalSupply = AMKT.totalSupply();
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            assertGe(
                token.balanceOf(MULTISIG),
                fmul(tokens[i].units + 1, totalSupply) + 1
            );
        }
    }

    // check that the deployed contracts match what we expect as a result of core deploy script
    function checkCoreDeploy() public {
        assertEq(address(AMKT), AMKT_PROXY);
        assertEq(address(vault), address(issuance.vault()));
        assertEq(address(issuance), address(vault.issuance()));
        assertEq(address(invokeableBounty), address(vault.rebalancer()));
        assertEq(
            address(activeBounty),
            address(invokeableBounty.activeBounty())
        );
        assertEq(address(AMKT), address(invokeableBounty.indexToken()));
        assertEq(address(AMKT), address(vault.indexToken()));
        assertEq(vault.inflationRate(), 0);
        assertEq(vault.emergencyResponder(), MULTISIG);
        assertEq(vault.emergency(), false);
        assertEq(vault.pendingOwner(), MULTISIG);
    }
}
