#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod farming {
    use farming::traits::farming::*;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct FarmingContract {
        #[storage_field]
        farming: Data,
    }

    impl Farming for FarmingContract {}

    impl FarmingContract {
        #[ink(constructor)]
        pub fn new(arsw_token: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.farming.arsw_token = arsw_token;
            })
        }
    }
}
