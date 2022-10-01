import {getWallet, setupContract, attachContract} from './helper'
import { expect } from "chai";
import * as BN from "bn.js";

const ONE = new BN(10).pow(new BN(18))

describe('ROUTER', () => {
    async function setup() {
        const wallet = await getWallet()
        const tokenA = await setupContract('psp22_token', 'new', new BN(ONE.muln(10000)))
        const tokenB = await setupContract('psp22_token', 'new', new BN(ONE.muln(10000)))
        const pair = await setupContract('pair_contract', 'new')
        const pair_code_hash = (await pair.abi).source.hash
        const factory_contract =  await setupContract('factory_contract', 'new', wallet.address, pair_code_hash)
        await factory_contract.contract.tx["factory::createPair"](tokenA.contract.address, tokenB.contract.address)
        const pair_address = await factory_contract.query["factory::getPair"](tokenA.contract.address, tokenB.contract.address)
        const pair_contract = await attachContract('pair_contract', pair_address.output.unwrap())
        const token0Address = await pair_contract.query["pair::getToken0"]()
        const match = tokenA.contract.address.toString() == token0Address.output.toString()
        const token0 = match ? tokenA : tokenB
        const token1 = match ? tokenB : tokenA
        const router_contract = await setupContract('router_contract', 'new', factory_contract.contract.address)

        return {
            wallet,
            deployer: factory_contract.deployer,
            token0: token0.contract,
            token1: token1.contract,
            pair: pair_contract.contract,
            factory: factory_contract.contract,
            router: router_contract.contract,
        }
    }

    it('quote', async () => {
        const { router } = await setup()

        // success case
        let tokenAAmount = ONE
        let tokenAReserve = ONE
        let tokenBReserve = ONE
        let res = await router.query["quote"](tokenAAmount, tokenAReserve, tokenBReserve)
        expect(JSON.stringify(res.output.toHuman())).to.equal(JSON.stringify({ Ok: '1,000,000,000,000,000,000' }))

        // success case
        tokenAAmount = ONE
        tokenAReserve = ONE.divn(2)
        tokenBReserve = ONE.muln(2)
        res = await router.query["quote"](tokenAAmount, tokenAReserve, tokenBReserve)
        expect(JSON.stringify(res.output.toHuman())).to.equal(JSON.stringify({ Ok: "4,000,000,000,000,000,000" }))

        tokenAAmount = ONE.muln(1000)
        tokenAReserve = ONE.muln(10)
        tokenBReserve = ONE.muln(1000)
        res = await router.query["quote"](tokenAAmount, tokenAReserve, tokenBReserve)
        expect(JSON.stringify(res.output.toHuman())).to.equal(JSON.stringify({ Ok: "100,000,000,000,000,000,000,000" }))
    })

    it('get_amount_out', async () => {
        const { router } = await setup()

        let amountIn = ONE.muln(1000)
        let reserveIn = ONE.muln(1000)
        let reserveOut = ONE.muln(1000)
        let res = await router.query["getAmountOut"](amountIn, reserveIn, reserveOut)
        expect(JSON.stringify(res.output.toHuman())).to.equal(JSON.stringify({ Ok: "499,248,873,309,964,947,421" }))
    })

    // it('get_amount_in', async () => {
        // const { contract } = await setup()

        // let amountOut = ONE.muln(100)
        // let reserveIn = ONE.muln(1000)
        // let reserveOut = ONE.muln(1000)
        // let res = await contract.query["getAmountIn"](amountOut, reserveIn, reserveOut)
        // expect(JSON.stringify(res.output.toHuman())).to.equal(JSON.stringify({ Ok: "" }))
    // })

    it('add_liquidity', async () => {
        const { wallet, token0, token1, pair, router } = await setup()

        // transfer token to account
        const token0Amount = ONE.muln(100)
        const token1Amount = ONE.muln(100)
        await expect(token0.tx['psp22::transfer'](pair.address, token0Amount, [])).to.eventually.be.fulfilled
        await expect(token1.tx['psp22::transfer'](pair.address, token1Amount, [])).to.eventually.be.fulfilled

        // approve router contract

        // add liquidity
    })
})
