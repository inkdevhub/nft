#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pair {
    use openbrush::traits::Storage;
    use ink_storage::traits::SpreadAllocate;
    use uniswap_v2::impls::pair::*;
    use uniswap_v2::traits::pair::*;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PairContract {
        #[storage_field]
        pair: data::Data,
    }

    impl Pair for PairContract {}

    impl PairContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}