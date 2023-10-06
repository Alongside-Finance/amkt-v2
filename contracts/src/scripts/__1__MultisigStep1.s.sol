pragma solidity =0.8.18;

import {Script} from "forge-std/Script.sol";
import {TokenInfo} from "src/Common.sol";
import {InvokeableBounty} from "src/invoke/Bounty.sol";
import {Bounty} from "src/interfaces/IInvokeableBounty.sol";
import {ActiveBounty} from "src/invoke/ActiveBounty.sol";
import {Vault} from "src/Vault.sol";
import {MULTISIG, InitialBountyHelper} from "src/scripts/Config.sol";

contract MultisigStep1Script is Script {
    function run(
        address _vault,
        address _invokeableBounty,
        address _activeBounty
    ) public virtual {
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();
        vm.startBroadcast(MULTISIG);
        Bounty memory _bountyToSet = Bounty({
            infos: tokens,
            fulfiller: MULTISIG,
            salt: keccak256(abi.encode(block.timestamp)),
            deadline: block.timestamp + 1
        });

        bytes32 hashToSet = InvokeableBounty(_invokeableBounty).hashBounty(
            _bountyToSet
        );
        ActiveBounty(_activeBounty).setHash(hashToSet);
        Vault(_vault).acceptOwnership();
        InvokeableBounty(_invokeableBounty).fulfillBounty(_bountyToSet, false);
        vm.stopBroadcast();
    }
}
