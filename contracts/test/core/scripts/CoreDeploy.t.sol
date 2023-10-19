pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {AMKT_PROXY, MULTISIG, FEE_RECEIPIENT} from "src/scripts/Config.sol";
import {IndexToken} from "src/IndexToken.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {CoreDeployScript} from "src/scripts/CoreDeploy.s.sol";
import {IndexToken} from "src/IndexToken.sol";
import {Vault} from "src/Vault.sol";
import {InvokeableBounty} from "src/invoke/Bounty.sol";
import {Bounty} from "src/interfaces/IInvokeableBounty.sol";
import {ActiveBounty} from "src/invoke/ActiveBounty.sol";
import {AlongsideGovernor} from "src/Governor.sol";
import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";
import {Issuance} from "src/invoke/Issuance.sol";

contract CoreDeployTest is Test {
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

    function setUp() public virtual {
        forkMainnet();
        deployContracts();
    }

    function forkMainnet() internal {
        string memory MAINNET_RPC_URL = vm.envString("MAINNET_RPC");
        uint256 TARGET_BLOCK = 17949557;
        uint256 mainnetFork = vm.createFork(MAINNET_RPC_URL);
        vm.selectFork(mainnetFork);
        vm.rollFork(TARGET_BLOCK);
        AMKT = IndexToken(AMKT_PROXY);
    }

    function deployContracts() internal {
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
    }

    function testGovernanceConfig() public {
        uint256 AVG_BLOCK_TIME = 12;
        assertEq(governor.votingDelay(), 1 days / AVG_BLOCK_TIME);
        assertEq(governor.votingPeriod(), 4 days / AVG_BLOCK_TIME);
        assertEq(governor.proposalThreshold(), 100e18);
        assertEq(governor.quorumNumerator(), 250);
        assertEq(governor.quorumDenominator(), 10000);
        assertEq(governor.timelock(), address(timelockController));
        assertEq(timelockController.getMinDelay(), 4 days);
        assertEq(governor.name(), "Alongside Governor");
        assertEq(governor.version(), "1");
    }

    function testTimelockConfig() public {
        assertEq(timelockController.getMinDelay(), 4 days);
        assertEq(
            timelockController.hasRole(
                timelockController.EXECUTOR_ROLE(),
                MULTISIG
            ),
            true
        );
        assertEq(
            timelockController.hasRole(
                timelockController.PROPOSER_ROLE(),
                MULTISIG
            ),
            true
        );
        assertEq(
            timelockController.hasRole(
                timelockController.CANCELLER_ROLE(),
                MULTISIG
            ),
            true
        );
        assertEq(
            timelockController.hasRole(
                timelockController.PROPOSER_ROLE(),
                address(governor)
            ),
            true
        );
    }

    function testVaultState() public {
        assertEq(vault.underlyingLength(), 0);
        assertEq(vault.issuance(), address(issuance));
        assertEq(vault.rebalancer(), address(invokeableBounty));
        assertEq(vault.feeRecipient(), FEE_RECEIPIENT);
        assertEq(vault.emergencyResponder(), MULTISIG);
        assertEq(vault.emergency(), false);
        assertEq(address(vault.indexToken()), address(AMKT));
        assertEq(vault.inflationRate(), 0);
        assertEq(vault.owner(), address(this));
        assertEq(vault.pendingOwner(), MULTISIG);
        assertNotEq(AMKT.minter(), address(vault));
    }
}
