# Upgrade

### Notes

Once the vault is prepared, the Multisig is also responsible for upgrading & initializing the token, as well as transferring power to Governance. After this step, we expect:

1. AMKT will have been upgraded, and that users are able to mint AMKT by bringing underlying assets, and receive underlying assets by redeeming AMKT.
1. Upon acceptance of the role by governance, governance (with a timelock) will have admin power the ProxyAdmin, Vault, and Bounty.

### Script

`__2__MultisigStep2.s.sol`: Steps to be run by the privileged Multisig address. This script cannot be run directly. Instead, it will be submitted as a bundle through Gnosis Safe.
