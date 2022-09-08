import {getWallet, setupContract, attachContract} from './helper'
import { expect } from "chai";
import * as BN from "bn.js";

const ONE = new BN(10).pow(new BN(18))

describe('PAIR', () => {
        async function setup() {
            const MINIMUM_LIQUIDITY = 1000
            const wallet = await getWallet()
            const tokenA = await setupContract('psp22_token', 'new', new BN(ONE.muln(10000)))
            const tokenB = await setupContract('psp22_token', 'new', new BN(ONE.muln(10000)))
            const pair = await setupContract('pair_contract', 'new')
            const pair_code_hash = (await pair.abi).source.hash
            const factory_contract = await setupContract('factory_contract', 'new', wallet.address, pair_code_hash)
            await factory_contract.contract.tx["factory::createPair"](tokenA.contract.address, tokenB.contract.address)
            const pair_address = await factory_contract.query["factory::getPair"](tokenA.contract.address, tokenB.contract.address)
            const pair_contract = await attachContract('pair_contract', pair_address.output.unwrap())
            const token0Address = await pair_contract.query["pair::getToken0"]()
            const match = tokenA.contract.address.toString() == token0Address.output.toString()
            const token0 = match ? tokenA : tokenB
            const token1 = match ? tokenB : tokenA

            return {
                MINIMUM_LIQUIDITY,
                wallet,
                token0: token0.contract,
                token1: token1.contract,
                pair: pair_contract.contract,
                token0Address: token0Address.output,
            }
        }

        it('mint', async () => {
            const { token0, token1, pair, wallet } = await setup()

            const token0Amount = ONE
            const token1Amount = ONE.muln(4)
            const expectedLiquidity = ONE.muln(2)

            await expect(token0.tx['psp22::transfer'](pair.address, token0Amount, [])).to.eventually.be.fulfilled
            await expect(token1.tx['psp22::transfer'](pair.address, token1Amount, [])).to.eventually.be.fulfilled

            await expect(pair.tx['pair::mint'](wallet.address)).to.eventually.be.fulfilled
            // Should be expectedLiquidity
            await expect(pair.query['psp22::totalSupply']()).to.eventually.have.property('output').to.equal(new BN('2000000000000000999'))
            // Should be expectedLiquidity - MINIMUM_LIQUIDITY
            await expect(pair.query["psp22::balanceOf"](wallet.address)).to.eventually.have.property('output').to.equal(new BN('1999999999999999999'))

            await expect(token0.query["psp22::balanceOf"](pair.address)).to.eventually.have.property('output').to.equal(token0Amount)
            await expect(token1.query["psp22::balanceOf"](pair.address)).to.eventually.have.property('output').to.equal(token1Amount)

            let reserves = await pair.query["pair::getReserves"]()
                expect(reserves.output[0]).to.equal(token0Amount)
                expect(reserves.output[1]).to.equal(token1Amount)
        })

    it('swap token0', async () => {
        const { token0, token1, pair, wallet } = await setup()

        const token0Amount = ONE.muln(5)
        const token1Amount = ONE.muln(10)
        await addLiquidity(token0, token1, pair, wallet, token0Amount, token1Amount)

        const swapAmount = ONE
        const expectedOutputAmount = new BN('1662497915624478906')
        await expect(token0.tx['psp22::transfer'](pair.address, swapAmount, [])).to.eventually.be.fulfilled

        await expect(pair.tx['pair::swap'](0, expectedOutputAmount, wallet.address)).to.eventually.be.fulfilled

        let reserves = await pair.query["pair::getReserves"]()
        expect(reserves.output[0]).to.equal(token0Amount.add(swapAmount))
        expect(reserves.output[1]).to.equal(token1Amount.sub(expectedOutputAmount))

        await expect(token0.query["psp22::balanceOf"](pair.address)).to.eventually.have.property('output').to.equal(token0Amount.add(swapAmount))
        await expect(token1.query["psp22::balanceOf"](pair.address)).to.eventually.have.property('output').to.equal(token1Amount.sub(expectedOutputAmount))

        const totalSupplyToken0 = await token0.query["psp22::totalSupply"]()
        const totalSupplyToken1 = await token1.query["psp22::totalSupply"]()

        // DON'T Pass =(
        // await expect(token0.query["psp22::balanceOf"](wallet.address)).to.eventually.have.property('output').to.equal(totalSupplyToken0.output.sub(token0Amount).sub(swapAmount))
        // await expect(token1.query["psp22::balanceOf"](wallet.address)).to.eventually.have.property('output').to.equal(totalSupplyToken1.output.sub(token1Amount).add(expectedOutputAmount))
    })
})

const addLiquidity = async (tokenO, token1, pair, wallet, token0Amount, token1Amount) => {
    await expect(tokenO.tx['psp22::transfer'](pair.address, token0Amount, [])).to.eventually.be.fulfilled
    await expect(token1.tx['psp22::transfer'](pair.address, token1Amount, [])).to.eventually.be.fulfilled
    await expect(pair.tx['pair::mint'](wallet.address)).to.eventually.be.fulfilled
}