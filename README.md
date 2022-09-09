# Astar WASM contracts examples
This repository contains examples of ink! contracts that can be deployed on Atar netwroks

### Contribute to this repository
contributions are welcome:
- If you find an issue or a refactor idea please open an issue
- If you want to add your own example open a Pull Request

### Contracts
#### Uniswap-V2
This folder contains the line by line implementation of [uniswap-v2 core](https://github.com/Uniswap/v2-core) & its tests. It uses [ink! 3.3.0](https://github.com/paritytech/ink/tree/v3.3.0) & [Openbrush 2.2.0](https://github.com/Supercolony-net/openbrush-contracts/tree/v2.2.0)

### DAO
On Chain governance Based on [Governor](https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/governance) contracts of OpenZeppelin

### Tests
These folders contain an example of how to use chain-extion structs in your contracts. The tests folders is an end-to-end tests for the chain extension.

**Runs the tests**
1. Run a local node \
   Use [swanky-node](https://github.com/AstarNetwork/swanky-node) or [Astar-local](https://github.com/AstarNetwork/Astar) that have the specified chain-extension enabled. Please follow the build & run instructions in their respective repository.
2. The end-to-end test uses redspot as testing environment. Node version should be 14
```bash
yarn install
npx redspot test
```