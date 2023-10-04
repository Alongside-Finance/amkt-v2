pragma solidity =0.8.15;

import {Script} from "forge-std/Script.sol";

import {IndexToken} from "src/IndexToken.sol";
import {Vault} from "src/Vault.sol";
import {Issuance} from "src/invoke/Issuance.sol";
import {InvokeableBounty} from "src/invoke/Bounty.sol";
import {ActiveBounty} from "src/invoke/ActiveBounty.sol";
import {AlongsideGovernor} from "src/Governor.sol";
import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";
import {IVotes} from "@openzeppelin/contracts/governance/utils/IVotes.sol";
import {AMKT, MULTISIG, FEE_RECEIPIENT, PROXY, PROXY_ADMIN, VERSION, NAME, SYMBOL, CANCELLATION_PERIOD} from "./Config.sol";
import {IIndexToken} from "src/interfaces/IIndexToken.sol";

contract CoreDeployScript is Script {
    struct DeployedContracts {
        Vault vault;
        Issuance issuance;
        InvokeableBounty invokeableBounty;
        ActiveBounty activeBounty;
        AlongsideGovernor governor;
        TimelockController timelockController;
        address newTokenImplementation;
        InvokeableBounty timelockInvokeableBounty;
        ActiveBounty timelockActiveBounty;
    }

    function run() public virtual returns (DeployedContracts memory) {
        vm.startBroadcast(msg.sender);

        DeployedContracts memory deployed;

        deployed.timelockController = new TimelockController(
            CANCELLATION_PERIOD,
            new address[](0),
            new address[](0),
            msg.sender
        );

        deployed.governor = new AlongsideGovernor(
            IVotes(AMKT),
            deployed.timelockController
        );

        deployed.timelockController.grantRole(
            deployed.timelockController.EXECUTOR_ROLE(),
            address(0)
        );
        deployed.timelockController.grantRole(
            deployed.timelockController.PROPOSER_ROLE(),
            MULTISIG
        );
        deployed.timelockController.grantRole(
            deployed.timelockController.CANCELLER_ROLE(),
            MULTISIG
        );
        deployed.timelockController.grantRole(
            deployed.timelockController.PROPOSER_ROLE(),
            address(deployed.governor)
        );
        deployed.timelockController.revokeRole(
            deployed.timelockController.TIMELOCK_ADMIN_ROLE(),
            msg.sender
        );

        (
            deployed.vault,
            deployed.issuance,
            deployed.invokeableBounty,
            deployed.activeBounty
        ) = deployVault({
            indexToken: IIndexToken(AMKT),
            _vaultOwner: MULTISIG,
            _bountySetter: MULTISIG,
            emergencyResponder: MULTISIG,
            inflationRate: 0,
            feeRecipient: FEE_RECEIPIENT
        });

        IndexToken newTokenImplementation = new IndexToken();
        newTokenImplementation.initialize(address(1));
        deployed.newTokenImplementation = address(newTokenImplementation);

        // Deploy another set of bounty contracts to transfer to timelock controller, since bounty contracts are immutable.
        (
            deployed.timelockInvokeableBounty,
            deployed.timelockActiveBounty
        ) = deployBounty(
            address(deployed.vault),
            address(deployed.timelockController)
        );

        vm.stopBroadcast();
        return deployed;
    }

    function deployVault(
        IIndexToken indexToken,
        address _vaultOwner,
        address _bountySetter,
        address emergencyResponder,
        uint256 inflationRate,
        address feeRecipient
    ) internal returns (Vault, Issuance, InvokeableBounty, ActiveBounty) {
        Vault _vault = new Vault({
            _indexToken: indexToken,
            _owner: msg.sender,
            _feeRecipient: feeRecipient,
            _emergencyResponder: emergencyResponder,
            _inflationRate: inflationRate
        });

        Issuance _issuance = deployIssuance(address(_vault));
        (
            InvokeableBounty _invokeableBounty,
            ActiveBounty _activeBounty
        ) = deployBounty(address(_vault), _bountySetter);

        _vault.setIssuance(address(_issuance));
        _vault.setRebalancer(address(_invokeableBounty));

        _vault.transferOwnership(_vaultOwner);

        return (_vault, _issuance, _invokeableBounty, _activeBounty);
    }

    function deployIssuance(address _vault) internal returns (Issuance) {
        Issuance issuance = new Issuance(_vault);

        return issuance;
    }

    function deployBounty(
        address _vault,
        address _bountySetter
    ) internal returns (InvokeableBounty, ActiveBounty) {
        ActiveBounty activeBounty = new ActiveBounty(_bountySetter);

        InvokeableBounty invokeableBounty = new InvokeableBounty({
            _vault: _vault,
            _activeBounty: address(activeBounty),
            _version: VERSION,
            _chainId: 1
        });

        return (invokeableBounty, activeBounty);
    }
}
