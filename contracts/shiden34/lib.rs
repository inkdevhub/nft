#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Enumerable, PSP34Metadata, PSP34Mintable, Ownable)]
#[openbrush::contract]
pub mod shiden34 {
    use openbrush::traits::Storage;
    use payable_mint_pkg::impls::payable_mint::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Shiden34 {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        enumerable: enumerable::Data,
        #[storage_field]
        payable_mint: types::Data,
    }

    #[overrider(PSP34Mintable)]
    #[openbrush::modifiers(only_owner)]
    fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
        psp34::InternalImpl::_mint_to(self, account, id)
    }

    impl payable_mint_pkg::impls::payable_mint::payable_mint::Internal for Shiden34 {}
    impl payable_mint::PayableMintImpl for Shiden34 {}

    impl Shiden34 {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
            ownable::Internal::_init_with_owner(&mut _instance, Self::env().caller());
            psp34::Internal::_mint_to(&mut _instance, Self::env().caller(), Id::U8(1))
                .expect("Can mint");
            let collection_id = psp34::PSP34Impl::collection_id(&_instance);
            metadata::Internal::_set_attribute(
                &mut _instance,
                collection_id.clone(),
                String::from("name"),
                String::from("Shiden34"),
            );
            metadata::Internal::_set_attribute(
                &mut _instance,
                collection_id,
                String::from("symbol"),
                String::from("SH34"),
            );
            _instance
        }
    }
}

// #![cfg_attr(not(feature = "std"), no_std)]
// #![feature(min_specialization)]

// #[openbrush::contract]
// pub mod shiden34 {
//     use openbrush::{
//         contracts::ownable::*,
//         contracts::psp34::extensions::{enumerable::*, metadata::*},
//         traits::{Storage, String},
//     };
//     use payable_mint_pkg::impls::payable_mint::*;
//     use payable_mint_pkg::traits::payable_mint::*;

//     #[ink(storage)]
//     #[derive(Default, Storage)]
//     pub struct Shiden34 {
//         #[storage_field]
//         psp34: psp34::Data<enumerable::Balances>,
//         #[storage_field]
//         ownable: ownable::Data,
//         #[storage_field]
//         metadata: metadata::Data,
//         #[storage_field]
//         payable_mint: types::Data,
//     }

//     impl PSP34 for Shiden34 {}
//     impl Ownable for Shiden34 {}
//     impl PSP34Enumerable for Shiden34 {}
//     impl PSP34Metadata for Shiden34 {}
//     impl PayableMint for Shiden34 {}

//     impl Shiden34 {
//         #[ink(constructor)]
//         pub fn new(
//             name: String,
//             symbol: String,
//             base_uri: String,
//             max_supply: u64,
//             price_per_mint: Balance,
//         ) -> Self {
//             let mut instance = Self::default();
//             instance._init_with_owner(instance.env().caller());
//             let collection_id = instance.collection_id();
//             instance._set_attribute(collection_id.clone(), String::from("name"), name);
//             instance._set_attribute(collection_id.clone(), String::from("symbol"), symbol);
//             instance._set_attribute(collection_id, String::from("baseUri"), base_uri);
//             instance.payable_mint.max_supply = max_supply;
//             instance.payable_mint.price_per_mint = price_per_mint;
//             instance.payable_mint.last_token_id = 0;
//             instance
//         }
//     }

//     #[cfg(test)]
//     mod tests {
//         use super::*;
//         use crate::shiden34::PSP34Error::*;
//         use ink::env::test;

//         const PRICE: Balance = 100_000_000_000_000_000;

//         fn init() -> Shiden34 {
//             const BASE_URI: &str = "ipfs://myIpfsUri/";
//             const MAX_SUPPLY: u64 = 10;
//             Shiden34::new(
//                 String::from("Shiden34"),
//                 String::from("SH34"),
//                 String::from(BASE_URI),
//                 MAX_SUPPLY,
//                 PRICE,
//             )
//         }

//         #[ink::test]
//         fn mint_multiple_works() {
//             let mut sh34 = init();
//             let accounts = test::default_accounts::<Environment>();
//             set_sender(accounts.bob);
//             let num_of_mints: u64 = 5;

//             assert_eq!(sh34.total_supply(), 0);
//             test::set_value_transferred::<ink::env::DefaultEnvironment>(
//                 PRICE * num_of_mints as u128,
//             );
//             assert!(sh34.mint(accounts.bob, num_of_mints).is_ok());
//             assert_eq!(sh34.total_supply(), num_of_mints as u128);
//             assert_eq!(sh34.balance_of(accounts.bob), 5);
//             assert_eq!(sh34.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
//             assert_eq!(sh34.owners_token_by_index(accounts.bob, 1), Ok(Id::U64(2)));
//             assert_eq!(sh34.owners_token_by_index(accounts.bob, 2), Ok(Id::U64(3)));
//             assert_eq!(sh34.owners_token_by_index(accounts.bob, 3), Ok(Id::U64(4)));
//             assert_eq!(sh34.owners_token_by_index(accounts.bob, 4), Ok(Id::U64(5)));
//             assert_eq!(
//                 sh34.owners_token_by_index(accounts.bob, 5),
//                 Err(TokenNotExists)
//             );
//         }

//         fn set_sender(sender: AccountId) {
//             ink::env::test::set_caller::<Environment>(sender);
//         }
//     }
// }
