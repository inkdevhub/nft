pub use crate::traits::rewarder::{
    data::Data,
    errors::RewarderError,
    getters::RewarderGetters,
};
use ink_env::AccountId;
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::traits::psp22::PSP22Ref,
    modifier_definition,
    modifiers,
    traits::{
        Balance,
        Storage,
    },
};

// Cannot be 0 or it will panic!
pub const REWARD_TOKEN_DIVISOR: u128 = 1_000_000_000_000_000_000;

#[openbrush::wrapper]
pub type RewarderRef = dyn Rewarder;

#[openbrush::trait_definition]
pub trait Rewarder: Storage<Data> + RewarderGetters {
    #[ink(message)]
    #[modifiers(only_master_chef)]
    fn on_arsw_reward(
        &self,
        _user: AccountId,
        to: AccountId,
        arsw_amount: Balance,
    ) -> Result<(), RewarderError> {
        let pending_reward = arsw_amount
            .checked_mul(self.reward_multiplier().into())
            .ok_or(RewarderError::MulOverflow1)?
            / REWARD_TOKEN_DIVISOR;
        let reward_token = self.data::<Data>().reward_token;
        let reward_bal = PSP22Ref::balance_of(&reward_token, Self::env().account_id());
        if pending_reward < reward_bal {
            PSP22Ref::transfer(&reward_token, to, reward_bal, Vec::new())?;
        } else {
            PSP22Ref::transfer(&reward_token, to, pending_reward, Vec::new())?;
        }
        Ok(())
    }

    #[ink(message)]
    fn pending_tokens(
        &self,
        _pool_id: u32,
        _user: AccountId,
        arsw_amount: Balance,
    ) -> Result<(AccountId, Balance), RewarderError> {
        let reward_amount = arsw_amount
            .checked_mul(self.reward_multiplier().into())
            .ok_or(RewarderError::MulOverflow2)?
            / REWARD_TOKEN_DIVISOR;
        Ok((self.reward_token(), reward_amount))
    }
}

#[modifier_definition]
pub fn only_master_chef<T, F, R, E>(instance: &T, body: F) -> Result<R, E>
where
    T: Storage<Data>,
    F: FnOnce(&T) -> Result<R, E>,
    E: From<RewarderError>,
{
    if instance.data().master_chef != T::env().caller() {
        return Err(From::from(RewarderError::CallerIsNotMasterChef))
    }
    body(instance)
}
