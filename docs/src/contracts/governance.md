# Governance

## Alongside Governor

We've adopted OpenZeppelin's [Governor](https://docs.openzeppelin.com/contracts/4.x/api/governance) standard for AMKT governance, allowing AMKT holders to govern the protocol.

## Timelock Controller

Notably, in order to make changes to the protocol (besides powers given to the Emergency Responder in [Vault](./contracts/vault.md)), it must go through a mandatory Timelock delay. This allows any user to selectively opt-out of the system before changes are made.
