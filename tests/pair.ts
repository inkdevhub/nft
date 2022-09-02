import { forceEras, setupContract } from './helper'
import { network } from 'redspot'
import { expect } from "./setup/chai";
import { buildTx } from '@redspot/patract/buildTx'
const { api } = network

describe('PAIR', () => {
    async function setup() {
        return await setupContract('pair_contract', 'new')
    }

    it('should read current era', async () => {
        const { contract } = await setup()

    })
})