import type { NextPage } from 'next'
import { useState } from 'react';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api'
import { Abi, ContractPromise } from '@polkadot/api-contract'

import Head from 'next/head'
import styles from '../styles/Home.module.css'
import abiData from './abi'

const WS_PROVIDER = 'ws://127.0.0.1:9944'
const gasLimit = 18750000000;
const storageDepositLimit = null;

const Home: NextPage = () => {
  const [address, setAddress] = useState('');
  const [addressSubmitted, setAddressSubmitted] = useState(false);
  const [value, setValue] = useState('');

  const query = async (contract: ContractPromise, address: string) => {
    // (We perform the send from an account, here using Alice's address)
    const { gasRequired, result, output } = await contract.query.get(
      address,
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

      if (output) {
        setValue(output?.toString());
      }
    } else {
      console.error('Error', result.asErr);
    }
  }

  const flip = async () => {
    const provider = new WsProvider(WS_PROVIDER);
		const api = new ApiPromise({ provider });

    await api.isReady;

    const keyring = new Keyring({ type: 'sr25519' });

    const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });

    console.log('API is ready');

    const abi = new Abi(abiData, api.registry.getChainProperties());

    const contract = new ContractPromise(api, abi, address);

    // Send the transaction, like elsewhere this is a normal extrinsic
    // with the same rules as applied in the API (As with the read example,
    // additional params, if required can follow)
    await contract.tx
      .flip({ storageDepositLimit, gasLimit })
      .signAndSend(alice, async (res) => {
        if (res.status.isInBlock) {
          console.log('in a block');
        } else if (res.status.isFinalized) {
          console.log('finalized');
        }
      });

    await query(contract, alice.address);
  }


  return (
    <div className={styles.container}>
      <Head>
        <title>Flipper Contract</title>
        <meta name="description" content="Flipper Contract" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        {addressSubmitted ? <>
          <h3 className={styles.title}>
            Flipper Contract
          </h3>

          <button onClick={flip}>Flip</button>

          <h4>{value}</h4>
        </> :
        <>
          <h3 className={styles.title}>
            Provide Contract Address
          </h3>
          <div className={styles.address}>
            <input
              type="text"
              value={address}
              onChange={e => setAddress(e.target.value)}
            />
            <button onClick={e => setAddressSubmitted(true)}>Set</button>
          </div>
        </>}
      </main>
    </div>
  )
}

export default Home
