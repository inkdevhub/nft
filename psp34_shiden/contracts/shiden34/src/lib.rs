#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod shiden34 {
    // imports from ink!
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;

    // imports from openbrush
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::extensions::{
                burnable::*,
                enumerable::*,
                metadata::*,
                mintable::*,
            },
            reentrancy_guard::*,
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Shiden34Contract {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        ownable: ownable::Data,

        // contract specific
        last_token_id: u64,
        collection_id: u32,
        max_supply: u64,
        price_per_mint: Balance,
    }

    // Section contains default implementation without any modifications
    impl PSP34 for Shiden34Contract {}
    impl PSP34Burnable for Shiden34Contract {}
    impl PSP34Mintable for Shiden34Contract {}
    impl PSP34Enumerable for Shiden34Contract {}
    impl PSP34Metadata for Shiden34Contract {}

    impl Shiden34Contract {
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            base_uri: String,
            max_supply: u64,
            price_per_mint: Balance,
        ) -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Shiden34Contract| {
                _instance._set_attribute(
                    Id::U8(0),
                    String::from("name").into_bytes(),
                    String::from(name).into_bytes(),
                );
                _instance._set_attribute(
                    Id::U8(0),
                    String::from("symbol").into_bytes(),
                    String::from(symbol).into_bytes(),
                );
                _instance._set_attribute(
                    Id::U8(0),
                    String::from("baseUri").into_bytes(),
                    String::from(base_uri).into_bytes(),
                );
                _instance.max_supply = max_supply;
                _instance.price_per_mint = price_per_mint;
                _instance.last_token_id = 0;
            })
        }

        /// Mint single token
        #[ink(message, payable)]
        #[modifiers(non_reentrant)]
        pub fn mint(&mut self) -> Result<(), PSP34Error> {
            self.check_value(1)?; //
            let caller = self.env().caller();
            self.last_token_id += 1;
            self._mint_to(caller, Id::U64(self.last_token_id))?;
            Ok(())
        }

        /// Mint several tokens
        #[ink(message, payable)]
        #[modifiers(non_reentrant)]
        pub fn mint_for(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error> {
            // self.data::<ownable::Data>().owner = _to;
            ink_env::debug_println!("####### mint RMRK contract amount:{:?}", mint_amount);
            if mint_amount == 0 {
                return Err(PSP34Error::Custom("CannotMintZeroTokens".to_string()))
            }
            // if self.data::<data::Data>().last_minted_token_id + mint_amount
            //     > self.data::<data::Data>().max_supply
            // {
            //     ink_env::debug_println!("####### error CollectionFullOrLocked");
            //     return Err(PSP34Error::CollectionFullOrLocked)
            // }
            // if Self::env().transferred_value()
            //     != mint_amount as u128 * self.data::<data::Data>().price_per_mint
            // {
            //     ink_env::debug_println!("####### error MintUnderpriced");
            //     return Err(PSP34Error::MintUnderpriced)
            // }
            self.check_value(mint_amount)?;

            let next_to_mint = self.last_token_id + 1; // first mint id is 1
            let mint_offset = next_to_mint + mint_amount;

            for mint_id in next_to_mint..mint_offset {
                ink_env::debug_println!("####### mint id:{:?}", mint_id);
                // mint in this contract
                assert!(self._mint_to(to, Id::U64(mint_id)).is_ok());
                self.last_token_id += 1;
            }

            Ok(())
        }

        /// Get max supply of tokens
        #[ink(message)]
        pub fn max_supply(&self) -> u64 {
            self.max_supply
        }

        /// Check id the transfered mint values is as expected
        fn check_value(&self, mint_amount: u64) -> Result<(), PSP34Error> {
            if Self::env().transferred_value() != mint_amount as u128 * self.price_per_mint {
                ink_env::debug_println!("####### error MintUnderpriced");
                return Err(PSP34Error::Custom("BadMintValue".to_string()))
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;
        const PRICE: Balance = 100_000_000_000_000_000;
        use ink_env::test;

        #[ink::test]
        fn init_works() {
            let sh34 = init();
            assert_eq!(
                sh34.get_attribute(Id::U8(0), String::from("name").into_bytes()),
                Some(String::from("Shiden34").into_bytes())
            );
        }

        fn init() -> Shiden34Contract {
            Shiden34Contract::new(
                String::from("Shiden34"),
                String::from("SH34"),
                String::from("ipfs://myIpfsUri/"),
                10,
                PRICE,
            )
        }

        #[ink::test]
        fn mint_single_works() {
            let mut sh34 = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);

            assert_eq!(sh34.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
            assert!(sh34.mint().is_ok());
            assert_eq!(sh34.total_supply(), 1);
            assert_eq!(sh34.owner_of(Id::U64(1)), Some(accounts.alice));
            assert_eq!(sh34.balance_of(accounts.alice), 1);
        }

        #[ink::test]
        fn mint_multiple_works() {
            let mut sh34 = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let num_of_mints: u64 = 5;

            assert_eq!(sh34.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert!(sh34.mint_for(accounts.bob, num_of_mints).is_ok());
            assert_eq!(sh34.total_supply(), num_of_mints as u128);
            assert_eq!(sh34.balance_of(accounts.bob), 5);
        }

        fn default_accounts() -> test::DefaultAccounts<ink_env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<Environment>(sender);
        }
    }
}
