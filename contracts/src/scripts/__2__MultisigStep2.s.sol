pragma solidity =0.8.18;

import {Script} from "forge-std/Script.sol";
import {MULTISIG, PROXY, PROXY_ADMIN, INFLATION_RATE} from "src/scripts/Config.sol";
import {ITransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {IndexToken} from "src/IndexToken.sol";
import {Vault} from "src/Vault.sol";

contract MultisigStep2Script is Script {
    function run(
        address _amkt,
        address _newImplementation,
        address _vault,
        address _timelockController,
        address _timelockInvokeableBounty
    ) public virtual {
        vm.startBroadcast(MULTISIG);
        ProxyAdmin(PROXY_ADMIN).upgradeAndCall(
            ITransparentUpgradeableProxy(PROXY),
            _newImplementation,
            abi.encodeWithSignature("initialize(address)", _vault)
        );
        Vault(_vault).setInflationRate(INFLATION_RATE);
        Vault(_vault).setRebalancer(_timelockInvokeableBounty);
        Vault(_vault).transferOwnership(_timelockController);
        ProxyAdmin(PROXY_ADMIN).transferOwnership(_timelockController);
        vm.stopBroadcast();
    }
}
