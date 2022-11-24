import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import Token_factory from '../types/constructors/psp22_token';
import Farming_factory from '../types/constructors/master_chef_contract';
import Token from '../types/contracts/psp22_token';
import Farming from '../types/contracts/master_chef_contract';
import { parseUnits, setupApi } from './setup';
import { expect } from 'chai';
describe('Farming', () => {
  let api: ApiPromise;
  let deployer: KeyringPair;
  let [arsw]: Token[] = [];
  let farming: Farming;
  let originBlock: number;

  async function setup(): Promise<void> {
    ({ api: api, alice: deployer } = await setupApi());
    const tokenFactory = new Token_factory(api, deployer);
    const { address: aploAddress } = await tokenFactory.new(
      parseUnits(1_000_000).toString(),
    );
    arsw = new Token(aploAddress, deployer, api);
    const farmingFactory = new Farming_factory(api, deployer);
    const { address: farmingAddress } = await farmingFactory.new(aploAddress);
    farming = new Farming(farmingAddress, deployer, api);
    ({ value: originBlock } = await farming.query.getFarmingOriginBlock());
  }
  describe('getPeriod', () => {
    it('successfully get 1st block of Period-1', async () => {
      await setup();
      const firstBlockOfPeriodOne = originBlock + 215_000;
      const {
        value: { ok: period },
      } = await farming.query.getPeriod(firstBlockOfPeriodOne);
      expect(period).to.equal(1);
    });

    it('successfully get medium block of Period-1', async () => {
      const blockPeriodOne = originBlock + 215_000 + 100_000;
      const {
        value: { ok: period },
      } = await farming.query.getPeriod(blockPeriodOne);
      expect(period).to.equal(1);
    });

    it('successfully get end block of Period-0', async () => {
      const endBlockOfPeriodZero = originBlock + 215_000 - 1;
      const {
        value: { ok: period },
      } = await farming.query.getPeriod(endBlockOfPeriodZero);
      expect(period).to.equal(0);
    });

    it('revert if the blockNumber is lower than the ARTHSWAP_ORIGIN_BLOCK', async () => {
      const blockBeforePeriodZero = originBlock - 1;
      const {
        value: { err },
      } = await farming.query.getPeriod(blockBeforePeriodZero);
      expect(err).to.have.property('blockNumberLowerThanOriginBlock');
    });
  });
});
