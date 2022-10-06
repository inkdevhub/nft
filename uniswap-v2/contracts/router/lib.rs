#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod router {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;
    use uniswap_v2::{
        impls::router::*,
        traits::router::*,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct RouterContract {
        #[storage_field]
        router: data::Data,
    }

    impl Router for RouterContract {}

    impl RouterContract {
        #[ink(constructor)]
        pub fn new(factory: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.router.factory = factory;
            })
        }
    }
}
