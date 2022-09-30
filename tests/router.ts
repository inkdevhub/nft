import {getWallet, setupContract} from './helper'
import { expect } from "chai";
import { encodeAddress } from "@polkadot/keyring"
import * as BN from "bn.js";

const ONE = new BN(10).pow(new BN(18))

describe('ROUTER', () => {
    async function setup() {
        const wallet = await getWallet()
        let pair = await setupContract('pair_contract', 'new')
        let pair_code_hash = (await pair.abi).source.hash
        let factory_contract =  await setupContract('factory_contract', 'new', wallet.address, pair_code_hash)
        let router_contract = await setupContract('router_contract', 'new', factory_contract.contract.address)

        return {
            wallet,
            deployer: factory_contract.deployer,
            alice: factory_contract.alice,
            contract: router_contract.contract,
        }
    }

    it('quote, getAmountIn, getAmountOut', async () => {
        const { contract, wallet } = await setup()

        let tokenAAmount = ONE
        let tokenAReserve = ONE
        let tokenBReserve = ONE
        let expectedAmount = ONE
        let res = await contract.query["quote"](tokenAAmount, tokenAReserve, tokenBReserve);
        console.log(res)
        await expect(contract.query["quote"](tokenAAmount, tokenAReserve, tokenBReserve)).to.eventually.have.property('output').to.equal(expectedAmount)

        tokenAAmount = ONE
        tokenAReserve = ONE.divn(2)
        tokenBReserve = ONE.muln(2)
        expectedAmount = ONE.muln(4)
        await expect(contract.query["quote"](tokenAAmount, tokenAReserve, tokenBReserve)).to.eventually.have.property('output').to.equal(expectedAmount)
    })
})
