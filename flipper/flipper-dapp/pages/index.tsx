import type { NextPage } from 'next'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { Abi, ContractPromise } from '@polkadot/api-contract'

import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'

const WS_PROVIDER = 'ws://127.0.0.1:9944'

const data = {
  "source": {
    "hash": "0xc6791d00ac69ebfe59fa5cfb7f55a5583ff7357da462a1c53cd8518a9cbee964",
    "language": "ink! 3.0.0",
    "compiler": "rustc 1.62.0-nightly"
  },
  "contract": {
    "name": "flipper",
    "version": "0.1.0",
    "authors": [
      "Nikhil Ranjan"
    ]
  },
  "V3": {
    "spec": {
      "constructors": [
        {
          "args": [
            {
              "label": "init_value",
              "type": {
                "displayName": [
                  "bool"
                ],
                "type": 0
              }
            }
          ],
          "docs": [
            "Constructor that initializes the `bool` value to the given `init_value`."
          ],
          "label": "new",
          "payable": false,
          "selector": "0x9bae9d5e"
        },
        {
          "args": [],
          "docs": [
            "Constructor that initializes the `bool` value to `false`.",
            "",
            "Constructors can delegate to other constructors."
          ],
          "label": "default",
          "payable": false,
          "selector": "0xed4b9d1b"
        }
      ],
      "docs": [],
      "events": [],
      "messages": [
        {
          "args": [],
          "docs": [
            " A message that can be called on instantiated contracts.",
            " This one flips the value of the stored `bool` from `true`",
            " to `false` and vice versa."
          ],
          "label": "flip",
          "mutates": true,
          "payable": false,
          "returnType": null,
          "selector": "0x633aa551"
        },
        {
          "args": [],
          "docs": [
            " Simply returns the current value of our `bool`."
          ],
          "label": "get",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "bool"
            ],
            "type": 0
          },
          "selector": "0x2f865bd9"
        }
      ]
    },
    "storage": {
      "struct": {
        "fields": [
          {
            "layout": {
              "cell": {
                "key": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "ty": 0
              }
            },
            "name": "value"
          }
        ]
      }
    },
    "types": [
      {
        "id": 0,
        "type": {
          "def": {
            "primitive": "bool"
          }
        }
      }
    ]
  }
};

const Home: NextPage = () => {

  const flip = async () => {
    const { web3Accounts, web3Enable, web3FromSource } = await import('@polkadot/extension-dapp');

    const address = '5EazoN6UvXJ2Zm8Kt1bcACS32fk2SmNLUcZ98njGx5uV2qd4'
    const sendAddress = '5CRvXUKwLDzFM1u9reZ1quf14pNbsw27EZWCBxPMmpydBfYV'
    const provider = new WsProvider(WS_PROVIDER);
		const api = new ApiPromise({ provider });

    await api.isReady;

    console.log('API is ready');

    const extensions = await web3Enable('Flipper');

    const allAccounts = await web3Accounts();

    console.log(allAccounts);

    const injected = await web3FromSource(allAccounts[0].meta.source);

		api.setSigner(injected.signer);

    const abi = new Abi(data, api.registry.getChainProperties());

    const contract = new ContractPromise(api, abi, address);

    const value = 0; // only for payable messages, call will fail otherwise
    const gasLimit = 18750000000;
    const storageDepositLimit = null;

    // Send the transaction, like elsewhere this is a normal extrinsic
    // with the same rules as applied in the API (As with the read example,
    // additional params, if required can follow - here only one is needed)
    await contract.tx
      .flip({ storageDepositLimit, gasLimit })
      .signAndSend(sendAddress, async (res) => {
        if (res.status.isInBlock) {
          console.log('in a block');
        } else if (res.status.isFinalized) {
          console.log('finalized');
        }

        // (We perform the send from an account, here using Alice's address)
        const { gasRequired, storageDeposit, result, output } = await contract.query.get(
          sendAddress,
          {
            gasLimit,
            storageDepositLimit,
          }
        );

        // The actual result from RPC as `ContractExecResult`
        console.log(result.toHuman());

        // the gas consumed for contract execution
        console.log(gasRequired.toHuman());

        // check if the call was successful
        if (result.isOk) {
          // output the return value
          console.log('Success', output?.toHuman());
        } else {
          console.error('Error', result.asErr);
        }
      });
  }


  return (
    <div className={styles.container}>
      <Head>
        <title>Flipper Contract</title>
        <meta name="description" content="Flipper Contract" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>
          Flipper Contract
        </h1>

        <button onClick={flip}>Flip</button>
      </main>
    </div>
  )
}

export default Home
