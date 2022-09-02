import { setupContract } from './helper'
import { network } from 'redspot'
import { expect } from "./setup/chai";
import { buildTx } from '@redspot/patract/buildTx'
const { api } = network

describe('FACTORY', () => {
    async function setup() {
        return await setupContract('factory_contract', 'new')
    }

    it('e', async () => {
        const { contract } = await setup()

    })
})