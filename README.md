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

