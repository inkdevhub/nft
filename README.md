[![workflow][a1]][a2] [![stack-exchange][s1]][s2] [![discord][l1]][l2] [![built-with-ink][i1]][i2]

[s1]: https://img.shields.io/badge/click-white.svg?logo=StackExchange&label=ink!%20Support%20on%20StackExchange&labelColor=white&color=blue
[s2]: https://substrate.stackexchange.com/questions/tagged/ink?tab=Votes
[a1]: https://github.com/swanky-dapps/nft/actions/workflows/test.yml/badge.svg
[a2]: https://github.com/swanky-dapps/nft/actions/workflows/test.yml
[l1]: https://img.shields.io/discord/722223075629727774?style=flat-square&label=discord
[l2]: https://discord.gg/Z3nC9U4
[i1]: /.images/ink.svg
[i2]: https://github.com/paritytech/ink

# NFT project using PSP34
This is an example nft project using ink! smart contract. The project is generated with Openbrush wizard for PSP34 with added PayableMinted trait.

### Purpose
This is an unaudited nft project template.
It can be used to speed up wasm nft project on Astar and other networks.

### License
Apache 2.0

### 🏗️ How to use - Contracts
##### 💫 Build
- Use this [instructions](https://use.ink/getting-started/setup) to setup your ink!/Rust environment

```sh
cargo +nightly contract build
```

##### 💫 Run unit test

```sh
cargo +nightly test
```
##### 💫 Deploy
First start your local node. Recommended [swanky-node](https://github.com/AstarNetwork/swanky-node) v0.13.0
```sh
./target/release/swanky-node --dev --tmp -lruntime=trace -lruntime::contracts=debug -lerror
```
- or deploy with polkadot.JS. Instructions on [Astar docs](https://docs.astar.network/docs/wasm/sc-dev/polkadotjs-ui)

##### 💫 Run integration test
First start your local node. Recommended [swanky-node](https://github.com/AstarNetwork/swanky-node) v0.13.0
And then:
```sh
yarn
yarn compile
yarn test
```

##### 💫 Deployed contracts
[Shiden Graffiti](https://github.com/Maar-io/ink-mint-dapp/tree/graffiti) - using ink! v3.4

