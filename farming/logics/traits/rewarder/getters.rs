use crate::traits::rewarder::rewarder::Data;
use ink_env::AccountId;
use openbrush::traits::Storage;

#[openbrush::trait_definition]
pub trait RewarderGetters: Storage<Data> {
    #[ink(message)]
    fn reward_multiplier(&self) -> u32 {
        self.data::<Data>().reward_multiplier
    }

    #[ink(message)]
    fn reward_token(&self) -> AccountId {
        self.data::<Data>().reward_token
    }

    #[ink(message)]
    fn master_chef(&self) -> AccountId {
        self.data::<Data>().master_chef
    }
}
