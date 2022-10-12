import type { NextPage } from 'next'
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api'
import { Abi, ContractPromise } from '@polkadot/api-contract'

import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'
import abiData from './abi'

const WS_PROVIDER = 'ws://127.0.0.1:9944'

const Home: NextPage = () => {

  const flip = async () => {
    const address = '5EazoN6UvXJ2Zm8Kt1bcACS32fk2SmNLUcZ98njGx5uV2qd4'
    const sendAddress = '5CRvXUKwLDzFM1u9reZ1quf14pNbsw27EZWCBxPMmpydBfYV'
    const provider = new WsProvider(WS_PROVIDER);
		const api = new ApiPromise({ provider });

    await api.isReady;

    const keyring = new Keyring({ type: 'sr25519' });

    const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });

    console.log('API is ready');

    const abi = new Abi(abiData, api.registry.getChainProperties());

    const contract = new ContractPromise(api, abi, address);

    const value = 0; // only for payable messages, call will fail otherwise
    const gasLimit = 18750000000;
    const storageDepositLimit = null;

    // Send the transaction, like elsewhere this is a normal extrinsic
    // with the same rules as applied in the API (As with the read example,
    // additional params, if required can follow - here only one is needed)
    await contract.tx
      .flip({ storageDepositLimit, gasLimit })
      .signAndSend(alice, async (res) => {
        if (res.status.isInBlock) {
          console.log('in a block');
        } else if (res.status.isFinalized) {
          console.log('finalized');
        }

        // (We perform the send from an account, here using Alice's address)
        const { gasRequired, storageDeposit, result, output } = await contract.query.get(
          alice.address,
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
