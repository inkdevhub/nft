use openbrush::traits::{
    AccountId,
    Balance,
};

#[openbrush::trait_definition]
pub trait FarmingEvents {
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
