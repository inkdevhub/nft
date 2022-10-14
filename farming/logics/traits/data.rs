use ink_prelude::vec::Vec;
use openbrush::{
    storage::Mapping,
    traits::AccountId,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    /// Address of ARSW contract.
    pub arsw_token: AccountId,

    /// Info of each MasterChef user.
    /// u128 `amount` LP token amount the user has provided.
    /// u128 `reward_debt` The amount of ARSW entitled to the user.
    /// key is (u32 `pool_id`, AccountId `user_address` )
    pub user_info: Mapping<(u32, AccountId), (u128, i128)>,

    /// Info of each MasterChef pool.
    /// u64 `alloc_point` The amount of allocation points assigned to the pool.
    /// Also known as the amount of ARSW to distribute per block.
    /// key is u32 `pool_id`
    pub pool_info: Mapping<u32, (u128, u64, u64)>,
    pub pool_info_length: u32,

    /// Address of the LP token for each MasterChef pool.
    pub lp_tokens: Vec<AccountId>,

    /// Address of each `rewarder` contract in MasterChef.
    pub rewarders: Vec<AccountId>,

    /// Total allocation points. Must be the sum of all allocation points in all pools.
    pub total_alloc_point: u32,
}
