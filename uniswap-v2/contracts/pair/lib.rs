#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pair {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            pausable::*,
        },
        traits::Storage,
    };
    use uniswap_v2::{
        impls::pair::*,
        traits::pair::*,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PairContract {
        #[storage_field]
        pause: pausable::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        pair: data::Data,
    }

    impl Pausable for PairContract {}

    impl Ownable for PairContract {}

    impl Pair for PairContract {}

    impl PairContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let caller = instance.env().caller();
                instance._init_with_owner(caller);
            })
        }
    }
}
