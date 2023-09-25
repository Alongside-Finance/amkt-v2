pragma solidity =0.8.15;

import "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {GnosisTest, GnosisTransaction} from "./Gnosis.t.sol";
import {CoreDeployScript} from "src/scripts/__0__CoreDeploy.s.sol";
import {InitialBountyHelper, AMKT, MULTISIG, PROXY_ADMIN, PROXY, FEE_SCALED, AMKT as AMKTAddress} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";
import {Bounty, InvokeableBounty} from "src/invoke/Bounty.sol";
import {ActiveBounty} from "src/invoke/ActiveBounty.sol";
import {Dealer} from "test/Dealer.t.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {IndexToken} from "src/IndexToken.sol";
import {Vault} from "src/Vault.sol";
import {AlongsideGovernor} from "src/Governor.sol";
import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";
import {Issuance} from "src/invoke/Issuance.sol";

contract UpgradeTest is GnosisTest {
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

    function setUp() public {
        vm.createSelectFork(vm.envString("MAINNET_RPC"));
        enableSimulation();

        CoreDeployScript.DeployedContracts
            memory deployed = setDeployedContracts();
        mockBalances(); // TODO: Remove when ready. Due before bundle submission.

        GnosisTransaction[] memory batch = createUpgradeBatch(deployed);
        bytes memory dataExecuted = executeBatch(batch);
        console.logBytes(dataExecuted); // raw data to be sent to gnosis
    }

    function setDeployedContracts()
        internal
        returns (CoreDeployScript.DeployedContracts memory)
    {
        AMKT = IndexToken(AMKTAddress);
        CoreDeployScript script = new CoreDeployScript(); // TODO: Remove when ready. Due before external review.
        CoreDeployScript.DeployedContracts memory deployed = script.run(); // TODO: Remove when ready. Due vefore external review.
        vault = deployed.vault;
        issuance = deployed.issuance;
        invokeableBounty = deployed.invokeableBounty;
        activeBounty = deployed.activeBounty;
        governor = deployed.governor;
        timelockController = deployed.timelockController;
        newTokenImplementation = deployed.newTokenImplementation;
        timelockInvokeableBounty = deployed.timelockInvokeableBounty;
        timelockActiveBounty = deployed.timelockActiveBounty;
        return deployed;
    }

    function mockBalances() internal {
        Dealer dealer = new Dealer();
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();
        for (uint256 i = 0; i < tokens.length; i++) {
            dealer.dealToken(
                tokens[i].token,
                MULTISIG,
                (tokens[i].units * AMKT.totalSupply()) / 1e18
            );
        }
    }

    function createUpgradeBatch(
        CoreDeployScript.DeployedContracts memory deployed
    ) public returns (GnosisTransaction[] memory) {
        // Initialize batch with known size
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens(); // TODO: Replace token units with real values. Due before submission.
        uint256 batchLength = tokens.length + 8;
        GnosisTransaction[] memory batch = new GnosisTransaction[](batchLength);

        // First 15 transactions are approving each token in the index for the issuance contract
        for (uint256 i = 0; i < 15; i++) {
            batch[i] = GnosisTransaction({
                to: tokens[i].token,
                data: abi.encodeWithSelector(
                    bytes4(keccak256("approve(address,uint256)")),
                    deployed.invokeableBounty,
                    2 ** 256 - 1
                )
            });
        }

        // Set hash to ActiveBounty for initial bounty
        Bounty memory _bountyToSet = Bounty({
            infos: tokens,
            salt: keccak256(abi.encode(block.timestamp)),
            deadline: block.timestamp + 1
        });

        bytes32 hashToSet = InvokeableBounty(deployed.invokeableBounty)
            .hashBounty(_bountyToSet);

        batch[15] = GnosisTransaction({
            to: address(deployed.activeBounty),
            data: abi.encodeWithSelector(
                bytes4(keccak256("setHash(bytes32)")),
                hashToSet
            )
        });

        // Accept ownership of Vault
        batch[16] = GnosisTransaction({
            to: address(deployed.vault),
            data: abi.encodeWithSelector(bytes4(keccak256("acceptOwnership()")))
        });

        // Fulfill initial bounty
        batch[17] = GnosisTransaction({
            to: address(deployed.invokeableBounty),
            data: abi.encodeWithSelector(
                bytes4(
                    keccak256(
                        "fulfillBounty(((address,uint256)[],uint256,bytes32),bool)"
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
                deployed.newTokenImplementation,
                abi.encodeWithSignature("initialize(address)", deployed.vault)
            )
        });

        // Set fee scaled
        batch[19] = GnosisTransaction({
            to: address(deployed.vault),
            data: abi.encodeWithSelector(
                bytes4(keccak256("setFeeScaled(uint256)")),
                FEE_SCALED
            )
        });

        // Set rebalancer to timeblock bounty
        batch[20] = GnosisTransaction({
            to: address(deployed.vault),
            data: abi.encodeWithSelector(
                bytes4(keccak256("setRebalancer(address)")),
                deployed.timelockInvokeableBounty
            )
        });

        // Transfer ownership to timelock
        batch[21] = GnosisTransaction({
            to: address(deployed.vault),
            data: abi.encodeWithSelector(
                bytes4(keccak256("transferOwnership(address)")),
                deployed.timelockController
            )
        });

        // Change proxy admin to timelock
        batch[22] = GnosisTransaction({
            to: PROXY_ADMIN,
            data: abi.encodeWithSelector(
                bytes4(keccak256("changeProxyAdmin(address,address)")),
                PROXY,
                deployed.timelockController
            )
        });
        return batch;
    }
}
