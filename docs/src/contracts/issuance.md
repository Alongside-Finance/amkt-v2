# Issuance

### Summary

Issuance is what users can interact with to freely mint or redeem AMKT.

### Notes

To mint or redeem AMKT, a user must pre-approve the tokens to the issuance contract before calling `issue(uint256)`. The tokens are minted to `msg.sender`, and likewise, redemptions send all `N` underlying to msg.sender.

Issuance is immutable. However, Timelock Controller is able to upgrade the issuance role through [Vault](./contracts/vault.md).
