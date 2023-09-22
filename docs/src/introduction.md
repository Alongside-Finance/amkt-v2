```


       d8888 888b     d888 888    d8P 88888888888                .d8888b.
      d88888 8888b   d8888 888   d8P      888                   d88P  Y88b
     d88P888 88888b.d88888 888  d8P       888                          888
    d88P 888 888Y88888P888 888d88K        888          888  888      .d88P
   d88P  888 888 Y888P 888 8888888b       888          888  888  .od888P"
  d88P   888 888  Y8P  888 888  Y88b      888          Y88  88P d88P"
 d8888888888 888   "   888 888   Y88b     888           Y8bd8P  888"
d88P     888 888       888 888    Y88b    888            Y88P   888888888



```

## Context

Code will be available on GitHub on 09/25/23.

AMKT is a fully backed market index, providing exposure to a market-cap weighted basket of assets, to be reconstituted quarterly.

AMKT is currently live [here](https://etherscan.io/token/0xf17a3fe536f8f7847f1385ec1bc967b2ca9cae8d). The underlying funds are custodied with Coinbase Prime.

The next iteration of AMKT moves custody on chain. Relying on [Vault](./contracts/vault.md) to custody underlying assets, and on token governance to submit accurate [bounties](./contracts/bounty.md) for the next set of underlying assets.

Please refer to the references section for [previous audits](./references/audits.md) and [deployed contracts](./references/deployed_contracts.md).

If you are a whitehat, please participate in our [Bug Bounty](./references/bug_bounty.md) program.

## Tests

Core tests can be found under `contracts/test/core`.

Migration tests using a Mainnet fork can be found under `contracts/test/migration`.
