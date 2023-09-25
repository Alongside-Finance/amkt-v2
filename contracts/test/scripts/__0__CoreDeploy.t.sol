pragma solidity =0.8.15;

import "forge-std/Test.sol";
import {AMKT as AMKTAddress} from "src/scripts/Config.sol";
import {IndexToken} from "src/IndexToken.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {CoreDeployScript} from "src/scripts/__0__CoreDeploy.s.sol";
import {IndexToken} from "src/IndexToken.sol";
import {Vault} from "src/Vault.sol";
import {Bounty, InvokeableBounty} from "src/invoke/Bounty.sol";
import {ActiveBounty} from "src/invoke/ActiveBounty.sol";
import {AlongsideGovernor} from "src/Governor.sol";
import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";
import {Issuance} from "src/invoke/Issuance.sol";

contract __0__CoreDeployTest is Test {
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
        AMKT = IndexToken(AMKTAddress);
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
}
