#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod token {
    use ink_prelude::string::String;
    use openbrush::{
        contracts::psp22::extensions::metadata::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyPSP22 {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP22 for MyPSP22 {}

    impl PSP22Metadata for MyPSP22 {}

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            instance.metadata.name = Some(String::from("UNI TOKEN"));
            instance.metadata.symbol = Some(String::from("UNI"));
            instance.metadata.decimals = 18;
            assert!(instance
                ._mint(instance.env().caller(), total_supply)
                .is_ok());
            instance
        }
    }
}
