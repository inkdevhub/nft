import {artifacts, network, patract} from 'redspot'
import BN from "bn.js";
import {createSigner} from "redspot/provider";
import { Keyring } from '@polkadot/keyring'
const {api} = network

const {getContractFactory, getRandomSigner} = patract
const {getSigners} = network

export const setupContract = async (name, constructor, ...args) => {
    await api.isReady
    const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]))
    const signers = await getSigners()
    const signer = await getRandomSigner(signers[0], one.muln(100000))
    // @ts-ignore
    const alice = createSigner(signer, new Keyring({ type: 'sr25519'}).addFromUri('//Alice'));
    const deployer = await getRandomSigner(signers[0], one.muln(100000))
    const bob = await getRandomSigner(signers[1], one.muln(100000))

    const contractFactory = await getContractFactory(name, deployer)
    const contract = await contractFactory.deploy(constructor, ...args)
    const abi = artifacts.readArtifact(name)

    return {
        deployer,
        alice,
        bob,
        accounts: [alice, await getRandomSigner(), await getRandomSigner()],
        contractFactory,
        contract,
        abi,
        one,
        query: contract.query,
        tx: contract.tx
    }
}