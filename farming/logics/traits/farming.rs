use ink_prelude::vec::Vec;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        Balance,
    },
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
    pub next_pool_info_id: u32,

    /// Address of the LP token for each MasterChef pool.
    pub lp_tokens: Vec<AccountId>,

    /// Address of each `rewarder` contract in MasterChef.
    pub rewarders: Vec<AccountId>,

    /// Total allocation points. Must be the sum of all allocation points in all pools.
    pub total_alloc_point: u32,
}

pub const ACC_ARSW_PRECISION: u8 = 12;
pub const ARTHSWAP_ORIGIN_BLOCK: u32 = 1u32;
pub const BLOCK_PER_PERIOD: u32 = 215000u32;
pub const MAX_PERIOD: u8 = 23u8;
pub const FIRST_PERIOD_REWERD_SUPPLY: Balance = 151629858171523000000u128;

#[openbrush::trait_definition]
pub trait Farming {
    #[ink(message)]
    fn add(&self) -> u32 {
        10u32
    }

    fn _emit_deposit_event(
        &self,
        _user: AccountId,
        _pool_id: u32,
        _amount: Balance,
        _to: AccountId,
    ) {
    }

    fn _emit_withdraw_event(
        &self,
        _user: AccountId,
        _pool_id: u32,
        _amount: Balance,
        _to: AccountId,
    ) {
    }

    fn _emit_emergency_withdraw_event(
        &self,
        _user: AccountId,
        _pool_id: u32,
        _amount: Balance,
        _to: AccountId,
    ) {
    }

    fn _emit_harvest_event(
        &self,
        _user: AccountId,
        _pool_id: u32,
        _amount: Balance,
        _to: AccountId,
    ) {
    }

    fn _emit_log_pool_addition_event(
        &self,
        _pool_id: u32,
        _alloc_point: u128,
        _lp_token: AccountId,
        _rewarder: AccountId,
    ) {
    }

    fn _emit_log_set_pool_event(
        &self,
        _pool_id: u32,
        _alloc_point: u128,
        _rewardes: AccountId,
        _overwrite: bool,
    ) {
    }

    fn _emit_log_update_pool_event(
        &self,
        _pool_id: u32,
        _last_reward_block: u32,
        _lp_supply: Balance,
        _acc_arsw_per_share: Balance,
    ) {
    }

    fn _emit_deposit_arsw_event(&self, _block_number: u32, _amount: Balance) {}
}
