use ink_prelude::vec::Vec;
use ink_storage::{
    traits::*,
    Mapping,
};
use openbrush::traits::AccountId;
use scale::{
    Decode,
    Encode,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Encode, Decode, SpreadLayout, PackedLayout, SpreadAllocate, Default)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct Pool {
    pub acc_arsw_per_share: u128,
    pub last_reward_block: u32,
    pub alloc_point: u32,
}

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    /// Address of ARSW contract.
    pub arsw_token: AccountId,

    /// Info of each MasterChef user.
    /// key (`pool_id`: u32, `user_address`: AccountId )
    /// Value (`amount`: u128, `reward_debt`: u128)
    /// `amount` LP token amount the user has provided.
    /// `reward_debt` The amount of ARSW entitled to the user.
    pub user_info: Mapping<(u32, AccountId), (u128, i128)>,

    /// Info of each MasterChef pool.
    /// Key `pool_id`: u32
    /// Value Pool (`acc_arsw_per_share`: u128, `last_reward_block`: u32, `alloc_point`: u64 )
    /// `alloc_point` The amount of allocation points assigned to the pool.
    /// Also known as the amount of ARSW to distribute per block.
    pub pool_info: Mapping<u32, Pool>,
    pub pool_info_length: u32,

    /// Address of the LP token for each MasterChef pool.
    pub lp_tokens: Vec<AccountId>,

    /// Address of each `rewarder` contract in MasterChef.
    pub rewarders: Mapping<u32, AccountId>,

    /// Total allocation points. Must be the sum of all allocation points in all pools.
    pub total_alloc_point: u32,
}
