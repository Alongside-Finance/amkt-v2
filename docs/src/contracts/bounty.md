# Bounty

### Summary

In order to facilitate rebalances, we utilize a bounty system. A trusted party (e.g. governance) confirms the nominals of the rebalance reflect market pricing around the time of submission. After some delay, any party can facilitate this rebalance.

### Notes

A bounty is computed as nominal (wei) token per `1e18` AMKT (so divide the onchain total supply by `1e18`, then divide the nominal token by that). It represents an entry in the next list of underlying for a given asset.

A bounty is hashed and stored in an `ActiveBounty` contract, it keeps track of the next allowed bounty.

Bounty is immutable. However, Timelock Controller is able to upgrade the rebalancer role through [Vault](./contracts/vault.md).

### Fullfillers

A fullfiller can be an EOA or a smart contract. In the former, the EOA will need to have the tokens preapproved for spending the by bounty contract. In the latter, the smart contract can request a callback where it can do its own custom logic for routing or whatever it needs.
