# Fund Migration

### Notes

The multisig is responsible for migrating funds from Coinbase Prime (where underlying funds for AMKT are currently custodied), to the Vault. They will have to calculate the new index allocation (including addresses and amounts), and use the Bounty system to transfer funds into the Vault. The goal of this step is to ensure that the Vault is fully collateralized, and that it is ready to be attached as the minter to the token.

Notably, we include bridged assets (e.g. [Wormhole SOL](https://etherscan.io/token/0xd31a59c85ae9d8edefec411d448f90841571b89c)) as part of the basket. The full list of assets (15, with 4 more in case rankings change from now until deployment) are included in `Config.sol` for review.

### Script

`__1__MultisigStep1.s.sol`: Steps to be run by the privileged Multisig address. This script cannot be run directly. Instead, it will be submitted as a bundle through Gnosis Safe.
