use ink_env::Hash;
use openbrush::traits::AccountId;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub factory: AccountId,
    // TODO: remove pair code hash as like https://github.com/Uniswap/v2-periphery/blob/master/contracts/libraries/UniswapV2Library.sol#L18
    pub pair_code_hash: Hash,
}
