import { setupContract } from './helper'
import { network } from 'redspot'
import { expect } from "./setup/chai";
import { buildTx } from '@redspot/patract/buildTx'
const { api } = network

describe('PAIR', () => {
    async function setup() {
        return await setupContract('pair_contract', 'new')
    }
})