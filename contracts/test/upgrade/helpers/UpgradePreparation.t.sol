pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {GnosisTest, GnosisTransaction} from "./Gnosis.t.sol";
import {CoreDeployScript} from "src/scripts/CoreDeploy.s.sol";
import {InitialBountyHelper, AMKT, MULTISIG, PROXY_ADMIN, PROXY, INFLATION_RATE, AMKT as AMKTAddress} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";
import {InvokeableBounty} from "src/invoke/Bounty.sol";
import {Bounty} from "src/interfaces/IInvokeableBounty.sol";
import {ActiveBounty} from "src/invoke/ActiveBounty.sol";
import {Dealer} from "test/Dealer.t.sol";
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
    InvokeableBounty timelockInvokeableBounty;
    ActiveBounty timelockActiveBounty;
    Quoter quoter;
    bytes batchExecutionData;

    function setUp() public virtual {
        vm.createSelectFork(vm.envString("MAINNET_RPC"), 18229914);
        enableSimulation();
        MIGRATION_WARNING_setDeployedContracts();
        MIGRATION_WARNING_mockSetDeployedContracts();
        MIGRATION_WARNING_warpForward(1 days + 2 hours); // there will be some time after we deploy the contracts, and it may be long.
        MIGRATION_WARNING_mockSafeBalances();
        checkSafeBalances();
        GnosisTransaction[] memory batch = createUpgradeBatch();
        batchExecutionData = getBatchExecutionData(batch);
        bool shouldPrintExecutionData = false;
        if (shouldPrintExecutionData) {
            console.logBytes(batchExecutionData);
        }
    }

    // WARNING: addresses must be updated before submission.
    // Expected date of finalization is October 30, 2023
    function MIGRATION_WARNING_setDeployedContracts() internal {
        AMKT = IndexToken(AMKTAddress);
        vault = Vault(address(0));
        issuance = Issuance(address(0));
        invokeableBounty = InvokeableBounty(address(0));
        activeBounty = ActiveBounty(address(0));
        governor = AlongsideGovernor(payable(address(0)));
        timelockController = TimelockController(payable(address(0)));
        newTokenImplementation = address(0);

        timelockActiveBounty = ActiveBounty(address(0));
        timelockInvokeableBounty = InvokeableBounty(address(0));
        quoter = Quoter(address(0));
    }

    function checkSafeBalances() internal {
        // check that the safe balance matches exactly what the initial bounty helper expects
        TokenInfo[] memory tokens = (new InitialBountyHelper())
            .MIGRATION_WARNING_tokens();
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            assertEq(
                token.balanceOf(MULTISIG),
                fmul(tokens[i].units + 1, AMKT.totalSupply()) + 1
            );
        }
    }

    function createUpgradeBatch() public returns (GnosisTransaction[] memory) {
        // Initialize batch with known size
        TokenInfo[] memory tokens = (new InitialBountyHelper())
            .MIGRATION_WARNING_tokens();
        uint256 batchLength = tokens.length + 8;
        GnosisTransaction[] memory batch = new GnosisTransaction[](batchLength);

        // First 15 transactions are approving each token in the index for the issuance contract
        for (uint256 i = 0; i < 15; i++) {
            batch[i] = GnosisTransaction({
                to: tokens[i].token,
                data: abi.encodeWithSelector(
                    bytes4(keccak256("approve(address,uint256)")),
                    invokeableBounty,
                    2 ** 256 - 1
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
                PROXY,
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

        // Set rebalancer to timeblock bounty
        batch[20] = GnosisTransaction({
            to: address(vault),
            data: abi.encodeWithSelector(
                bytes4(keccak256("setRebalancer(address)")),
                timelockInvokeableBounty
            )
        });

        // Transfer vault ownership to timelock
        batch[21] = GnosisTransaction({
            to: address(vault),
            data: abi.encodeWithSelector(
                bytes4(keccak256("transferOwnership(address)")),
                timelockController
            )
        });

        // Transfer proxyAdmin ownership to timelock
        batch[22] = GnosisTransaction({
            to: PROXY_ADMIN,
            data: abi.encodeWithSelector(
                bytes4(keccak256("transferOwnership(address)")),
                timelockController
            )
        });
        return batch;
    }

    // Mock helpers

    // WARNING: must be removed before submission.
    // Expected date of finalization is October 30, 2023
    function MIGRATION_WARNING_warpForward(uint256 sec) internal {
        _warpForward(sec);
    }

    // WARNING: must be removed before submission.
    // Expected date of finalization is October 30, 2023
    function MIGRATION_WARNING_mockSetDeployedContracts() internal {
        AMKT = IndexToken(AMKTAddress);
        CoreDeployScript script = new CoreDeployScript();
        CoreDeployScript.DeployedContracts memory deployed = script.run();
        vault = deployed.vault;
        issuance = deployed.issuance;
        invokeableBounty = deployed.invokeableBounty;
        activeBounty = deployed.activeBounty;
        governor = deployed.governor;
        timelockController = deployed.timelockController;
        newTokenImplementation = deployed.newTokenImplementation;
        timelockInvokeableBounty = deployed.timelockInvokeableBounty;
        timelockActiveBounty = deployed.timelockActiveBounty;
        quoter = deployed.quoter;
    }

    // WARNING: must be removed before submission.
    // Expected date of finalization is October 30, 2023
    function MIGRATION_WARNING_mockSafeBalances() internal {
        Dealer dealer = new Dealer();
        TokenInfo[] memory tokens = (new InitialBountyHelper())
            .MIGRATION_WARNING_tokens();
        for (uint256 i = 0; i < tokens.length; i++) {
            dealer.dealToken(
                tokens[i].token,
                MULTISIG,
                fmul(tokens[i].units + 1, AMKT.totalSupply()) + 1
            );
        }
    }
}
