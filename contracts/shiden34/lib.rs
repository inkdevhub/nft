#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod shiden34 {
    use openbrush::{
        contracts::ownable::*,
        contracts::psp34::extensions::{enumerable::*, metadata::*, mintable::*},
        traits::{Storage, String},
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Shiden34 {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP34 for Shiden34 {}
    impl Ownable for Shiden34 {}
    impl PSP34Mintable for Shiden34 {
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            self._mint_to(account, id)
        }
    }
    impl PSP34Enumerable for Shiden34 {}
    impl PSP34Metadata for Shiden34 {}

    impl Shiden34 {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(instance.env().caller());
            let collection_id = instance.collection_id();
            instance._set_attribute(
                collection_id.clone(),
                String::from("name"),
                String::from("Shiden34"),
            );
            instance._set_attribute(collection_id, String::from("symbol"), String::from("SH34"));
            instance
        }

        #[ink(message, payable)]
        pub fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            if Self::env().transferred_value() != 1_000_000_000_000_000_000 {
                return Err(PSP34Error::Custom(String::from("BadMintValue")));
            }
            self._mint_to(account, id)
        }
    }
}
