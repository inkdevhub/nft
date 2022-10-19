# Flipper: wasm dapp for astar

This is a demo for Simple WASM contract. Contract name is Flipper. Flipper contract has two method. One transaction method `flip` and one query method `get`. Flipper contract is meant to show hello world use case for wasm, swanky and connecting to contract via a react frontend.

`contract` folder contains the contract code `ui` folder contains the UI code. UI is written in next.js and react.

# Requirements

- node.js
- swanky cli https://github.com/AstarNetwork/swanky-cli

# Usage

Install swanky cli https://github.com/AstarNetwork/swanky-cli
- `$ npm install -g @astar-network/swanky-cli`

##### Deploy flipper contract
0. Init \
In the root workspace `Cargo.toml` uncomment the line `"flipper/flipper/contracts/*"` \
In `./flipper` folder run `swanky init flipper` and chose `flipper` as template and as contract name. Chose `Y` when asking to download swanky node.

1. Start the local node

- `cd flipper`
- `swanky node start`

2. Build the contract

`swanky contract compile flipper`

3. deploy the contract

`swanky contract deploy flipper --account alice -g 100000000000 -a true`

Note down the contract address.

##### Run the UI
Go to ui folder

- `cd ../ui`

Install Dependencies

- `yarn`

Start next.js server

- `yarn dev`

Go to http://localhost:3000 and enter the contract address. Flip button flips the boolean value.