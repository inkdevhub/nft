#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rewarder {
    use farming::traits::rewarder::{
        data::*,
        getters::*,
        rewarder::*,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Rewarderontract {
        #[storage_field]
        rewarder: Data,
    }

    impl Rewarder for Rewarderontract {}

    impl RewarderGetters for Rewarderontract {}

    impl Rewarderontract {
        #[ink(constructor)]
        pub fn new(
            reward_multiplier: u32,
            reward_token: AccountId,
            master_chef: AccountId,
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.rewarder.reward_multiplier = reward_multiplier;
                instance.rewarder.reward_token = reward_token;
                instance.rewarder.master_chef = master_chef;
            })
        }

        #[ink(message)]
        pub fn dummy(&self) {}
    }
}
