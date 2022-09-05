import {getWallet, setupContract, ONE} from './helper'
import { network } from 'redspot'
import { expect } from "chai";
import * as BN from "bn.js";
import {encodeAddress} from "@polkadot/keyring";
describe('PAIR', () => {
        async function setup() {
            const wallet = await getWallet()
            let token_1 = await setupContract('psp22_token', 'new', new BN(ONE.muln(10000)))
            let token_2 = await setupContract('psp22_token', 'new', new BN(ONE.muln(10000)))
            let pair = await setupContract('pair_contract', 'new')
            let pair_code_hash = (await pair.abi).source.hash
            let factory_contract = await setupContract('factory_contract', 'new', wallet.address, pair_code_hash)
            await factory_contract.contract.tx["factory::createPair"](token_1.contract.address, token_2.contract.address)
            let pair_address = await factory_contract.contract.query["factory::getPair"](token_1.contract.address, token_2.contract.address)
            // attach contract to use it

            return {
                wallet,
                deployer: factory_contract.deployer,
                alice: factory_contract.alice,
                contract: factory_contract.contract,
                token_1: token_1.contract,
                token_2: token_2.contract
            }
        }

        it('mint', async () => {
            const { contract, wallet } = await setup()

            const zero_address = "0x0000000000000000000000000000000000000000000000000000000000000000"
            await expect(contract.query["factory::feeTo"]()).to.eventually.have.property('output').to.equal(encodeAddress(zero_address))
            await expect(contract.query["factory::feeToSetter"]()).to.eventually.have.property('output').to.equal(wallet.address)
            await expect(contract.query["factory::allPairLength"]()).to.eventually.have.property('output').to.equal(0)
        })

})