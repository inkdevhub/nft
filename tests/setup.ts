import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
// Create a new instance of contract
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519' });
export async function setupApi(): Promise<{
  api: ApiPromise;
  alice: KeyringPair;
  bob: KeyringPair;
}> {
  const api = await ApiPromise.create({ provider: wsProvider });
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');
  return {
    api,
    alice,
    bob,
  };
}

export function parseUnits(amount: bigint | number, decimals = 18): bigint {
  return BigInt(amount) * 10n ** BigInt(decimals);
}
