# Flipper: wasm dapp for astar

This is a demo for Simple WASM contract. Contract name is Flipper. Flipper contract has two method. One transaction method `flip` and one query method `get`. Flipper contract is meant to show hello world use case for wasm, swanky and connecting to contract via a react frontend.

`contract` folder contains the contract code `ui` folder contains the UI code. UI is written in next.js and react.

# Requirements

- node.js
- swanky cli https://github.com/AstarNetwork/swanky-cli

# Usage

Install swanky cli https://github.com/AstarNetwork/swanky-cli
- `$ npm install -g @astar-network/swanky-cli`

contract folder was created by command: `swanky contract new CONTRACTNAME`. You might need to recreate contract folder for your environment.

Start the local node

- `cd contract`
- `swanky node start`

Deploy the contract

- `cd contract`
- `swanky contract deploy flipper --account alice -g 1000000000 -a 1`

Note down the contract address.

Go to ui folder

- `cd ../ui`

Install Dependencies

- `yarn`

Start next.js server

- `yarn dev`

Go to http://localhost:3000 and enter the contract address. Flip button flips the boolean value.