#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod master_chef_contract {
    use farming::traits::master_chef::{
        events::*,
        farming::*,
        getters::*,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable,
            ownable::Internal,
        },
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct FarmingContract {
        #[storage_field]
        farming: Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl Farming for FarmingContract {}

    impl FarmingGetters for FarmingContract {}

    impl FarmingEvents for FarmingContract {}

    impl FarmingContract {
        #[ink(constructor)]
        pub fn new(arsw_token: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let caller = instance.env().caller();
                instance._init_with_owner(caller);
                instance.farming.arsw_token = arsw_token;
                instance.farming.farming_origin_block = Self::env().block_number();
            })
        }
    }
}
