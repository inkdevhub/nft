import {getWallet, setupContract, attachContract} from './helper'
import { expect } from "chai";
import BN from "bn.js";

const Decimal = 18
const ONE = new BN(10).pow(new BN(Decimal))

describe('ROUTER', () => {
    async function setup() {
        const wallet = await getWallet()
        const wNative = await setupContract('w_native_token', 'new')
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
        const router_contract = await setupContract('router_contract', 'new', factory_contract.contract.address, wNative.contract.address, pair_code_hash)

        return {
            wallet,
            deployer: router_contract.deployer,
            token0: token0.contract,
            token1: token1.contract,
            pair: pair_contract.contract,
            factory: factory_contract.contract,
            router: router_contract.contract,
        }
    }

    it('quote', async () => {
        const { router } = await setup()

        let tokenAAmount = ONE
        let tokenAReserve = ONE
        let tokenBReserve = ONE
        let res = await router.query["router::quote"](tokenAAmount, tokenAReserve, tokenBReserve)
        expect(res.result.isOk).to.be.true
        if (res.result.isOk) {
            expect(parseInt(res.output.toJSON()["ok"], 16) / 10**Decimal).to.equal(1)
        }

        tokenAAmount = ONE
        tokenAReserve = ONE.divn(2)
        tokenBReserve = ONE.muln(2)
        res = await router.query["router::quote"](tokenAAmount, tokenAReserve, tokenBReserve)
        expect(res.result.isOk).to.be.true
        if (res.result.isOk) {
            expect(parseInt(res.output.toJSON()["ok"], 16) / 10**Decimal).to.equal(4)
        }

        tokenAAmount = ONE.muln(1000)
        tokenAReserve = ONE.muln(10)
        tokenBReserve = ONE.muln(1000)
        res = await router.query["router::quote"](tokenAAmount, tokenAReserve, tokenBReserve)
        expect(res.result.isOk).to.be.true
        if (res.result.isOk) {
            expect(BigInt(res.output.toJSON()["ok"]) / BigInt(10**Decimal)).to.equal(100000n)
        }

        tokenAAmount = ONE.muln(124)
        tokenAReserve = ONE.muln(234)
        tokenBReserve = ONE.muln(111)
        res = await router.query["router::quote"](tokenAAmount, tokenAReserve, tokenBReserve)
        expect(res.result.isOk).to.be.true
        if (res.result.isOk) {
            expect(BigInt(res.output.toJSON()["ok"]) / BigInt(10**Decimal)).to.equal(58n)
        }
    })

    it('get_amount_out', async () => {
        const { router } = await setup()

        let amountIn = ONE.muln(1000)
        let reserveIn = ONE.muln(1000)
        let reserveOut = ONE.muln(1000)
        let res = await router.query["router::getAmountOut"](amountIn, reserveIn, reserveOut)
        expect(res.result.isOk).to.be.true
        if (res.result.isOk) {
            expect(BigInt(res.output.toJSON()["ok"]) / BigInt(10**Decimal)).to.equal(499n)
        }
    })

    it('get_amount_in', async () => {
        const { router } = await setup()

        let amountOut = ONE.muln(450)
        let reserveIn = ONE.muln(742)
        let reserveOut = ONE.muln(867)
        let res = await router.query["router::getAmountIn"](amountOut, reserveIn, reserveOut)
        expect(res.result.isOk).to.be.true
        if (res.result.isOk) {
            expect(BigInt(res.output.toJSON()["ok"]) / BigInt(10**Decimal)).to.equal(803n)
        }
    })
})
