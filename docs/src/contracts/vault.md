# Vault

### Summary

Vault is where underlying assets of AMKT is custodied. It acts as the custodian of the index, where rules of movement of funds are transparent and codified.

### Notes

Vault tracks real units and virtual units. `realUnits()` are what you would get if you went and redeemed some of your index token right now. We use this system because fees will accrue inkind via inflation and this is a method to do that efficently.

Most of the user-facing functionality does not live here, instead it's through other contracts that interact with the Vault through external calls.

Timelock Controller is the owner of Vault, and has privileged functions.

Emergency Responder is able to turn off issuance and rebalances, in the case of an emergency.
