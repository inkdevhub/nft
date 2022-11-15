#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod shiden34 {
    // imports from ink!
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_prelude::string::{
        String as PreludeString,
        ToString,
    };
    use ink_storage::traits::SpreadAllocate;

    // imports from openbrush
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::{
                extensions::{
                    enumerable::*,
                    metadata::*,
                },
                Internal,
            },
            reentrancy_guard::*,
        },
        modifiers,
        traits::{
            Storage,
            String,
        },
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
    impl PSP34Enumerable for Shiden34Contract {}
    impl PSP34Metadata for Shiden34Contract {}
    impl Ownable for Shiden34Contract {}

    #[openbrush::trait_definition]
    pub trait PSP34Custom {
        #[ink(message, payable)]
        fn mint_next(&mut self) -> Result<(), PSP34Error>;
        #[ink(message, payable)]
        fn mint_for(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error>;
        #[ink(message)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), PSP34Error>;
        #[ink(message)]
        fn token_uri(&self, token_id: u64) -> Result<String, PSP34Error>;
        #[ink(message)]
        fn max_supply(&self) -> u64;
        #[ink(message)]
        fn price(&self) -> Balance;
        #[ink(message)]
        fn withdraw(&mut self) -> Result<(), PSP34Error>;

        // internal functions
        fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<(), PSP34Error>;
        fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error>;
        fn _token_exists(&self, id: Id) -> Result<(), PSP34Error>;
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

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
                let collection_id = _instance.collection_id();
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("name").into_bytes(),
                    name.into_bytes(),
                );
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("symbol").into_bytes(),
                    symbol.into_bytes(),
                );
                _instance._set_attribute(
                    collection_id,
                    String::from("baseUri").into_bytes(),
                    base_uri.into_bytes(),
                );
                _instance.max_supply = max_supply;
                _instance.price_per_mint = price_per_mint;
                _instance.last_token_id = 0;
                let caller = _instance.env().caller();
                _instance._init_with_owner(caller);
            })
        }

    impl psp34::Internal for Shiden34Contract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });

        fn _emit_approval_event(
            &self,
            from: AccountId,
            to: AccountId,
            id: Option<Id>,
            approved: bool,
        ) {
            self.env().emit_event(Approval {
                from,
                to,
                id,
                approved,
            });

    impl PSP34Custom for Shiden34Contract {
        /// Mint next available token for the caller
        #[ink(message, payable)]
        fn mint_next(&mut self) -> Result<(), PSP34Error> {
            self._check_value(self.env().transferred_value(), 1)?;
            let caller = self.env().caller();
            if let Some(token_id) = self.last_token_id.checked_add(1) {
                self.last_token_id += 1;
                self._mint_to(caller, Id::U64(token_id))?;
                return Ok(())
            }
            return Err(PSP34Error::Custom(String::from("CollectionFullOrLocked")))
        }

        /// Mint several tokens
        #[ink(message, payable)]
        #[modifiers(non_reentrant)]
        fn mint_for(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error> {
            self._check_value(self.env().transferred_value(), mint_amount)?;
            self._check_amount(mint_amount)?;

            let next_to_mint = self.last_token_id + 1; // first mint id is 1
            let mint_offset = next_to_mint + mint_amount;

            for mint_id in next_to_mint..mint_offset {
                self._mint_to(to, Id::U64(mint_id))?;
                self.last_token_id += 1;
            }

            Ok(())
        }

        /// Set new value for the baseUri
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), PSP34Error> {
            self._set_attribute(
                self.collection_id(),
                String::from("baseUri").into_bytes(),
                uri.into_bytes(),
            );
            Ok(())
        }

        /// Get URI from token ID
        #[ink(message)]
        fn token_uri(&self, token_id: u64) -> Result<String, PSP34Error> {
            self._token_exists(Id::U64(token_id))?;
            let value =
                self.get_attribute(self.collection_id(), String::from("baseUri").into_bytes());
            let mut token_uri = String::from_utf8(value.unwrap()).unwrap();
            token_uri = token_uri + &token_id.to_string() + &String::from(".json");
            Ok(token_uri)
        }

        /// Get max supply of tokens
        #[ink(message)]
        fn max_supply(&self) -> u64 {
            self.max_supply
        }

        /// Get token price
        #[ink(message)]
        fn price(&self) -> Balance {
            self.price_per_mint
        }

        /// Get max supply of tokens
        #[ink(message)]
        #[modifiers(only_owner)]
        fn withdraw(&mut self) -> Result<(), PSP34Error> {
            let balance = self.env().balance();
            let current_balance = balance
                .checked_sub(self.env().minimum_balance())
                .unwrap_or_default();
            self.env()
                .transfer(self.owner(), current_balance)
                .map_err(|_| PSP34Error::Custom(String::from("WithdrawFailed")))?;
            Ok(())
        }

        /// Check if the transferred mint values is as expected
        fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<(), PSP34Error> {
            if let Some(value) = (mint_amount as u128).checked_mul(self.price_per_mint) {
                if transfered_value == value {
                    return Ok(())
                }
            }

            return Err(PSP34Error::Custom("BadMintValue".to_string()))
        }

        /// Check amount of tokens to be minted
        fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error> {
            if mint_amount == 0 {
                return Err(PSP34Error::Custom("CannotMintZeroTokens".to_string()))
            }
            if let Some(amount) = self.last_token_id.checked_add(mint_amount) {
                if amount <= self.max_supply {
                    return Ok(())
                }
            }
            return Err(PSP34Error::Custom("CollectionFullOrLocked".to_string()))
        }

        /// Check if token is minted
        fn _token_exists(&self, id: Id) -> Result<(), PSP34Error> {
            self.owner_of(id).ok_or(PSP34Error::TokenNotExists)?;
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;
        const PRICE: Balance = 100_000_000_000_000_000;
        const BASE_URI: &str = "ipfs://myIpfsUri/";
        const MAX_SUPPLY: u64 = 10;
        use crate::shiden34::PSP34Error::*;
        use ink_env::test;

        #[ink::test]
        fn init_works() {
            let sh34 = init();
            let collection_id = sh34.collection_id();
            assert_eq!(
                sh34.get_attribute(collection_id.clone(), String::from("name").into_bytes()),
                Some(String::from("Shiden34").into_bytes())
            );
            assert_eq!(
                sh34.get_attribute(collection_id.clone(), String::from("symbol").into_bytes()),
                Some(String::from("SH34").into_bytes())
            );
            assert_eq!(
                sh34.get_attribute(collection_id, String::from("baseUri").into_bytes()),
                Some(String::from(BASE_URI).into_bytes())
            );
            assert_eq!(sh34.max_supply, MAX_SUPPLY);
            assert_eq!(sh34.price_per_mint, PRICE);
        }

        fn init() -> Shiden34Contract {
            Shiden34Contract::new(
                String::from("Shiden34"),
                String::from("SH34"),
                String::from(BASE_URI),
                MAX_SUPPLY,
                PRICE,
            )
        }

        #[ink::test]
        fn mint_single_works() {
            let mut sh34 = init();
            let accounts = default_accounts();
            assert_eq!(sh34.owner(), accounts.alice);
            set_sender(accounts.bob);

            assert_eq!(sh34.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
            assert!(sh34.mint_next().is_ok());
            assert_eq!(sh34.total_supply(), 1);
            assert_eq!(sh34.owner_of(Id::U64(1)), Some(accounts.bob));
            assert_eq!(sh34.balance_of(accounts.bob), 1);
            assert_eq!(sh34.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            assert_eq!(sh34.last_token_id, 1);
            assert_eq!(1, ink_env::test::recorded_events().count());
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
            assert_eq!(sh34.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            assert_eq!(sh34.owners_token_by_index(accounts.bob, 1), Ok(Id::U64(2)));
            assert_eq!(sh34.owners_token_by_index(accounts.bob, 2), Ok(Id::U64(3)));
            assert_eq!(sh34.owners_token_by_index(accounts.bob, 3), Ok(Id::U64(4)));
            assert_eq!(sh34.owners_token_by_index(accounts.bob, 4), Ok(Id::U64(5)));
            assert_eq!(5, ink_env::test::recorded_events().count());
            assert_eq!(
                sh34.owners_token_by_index(accounts.bob, 5),
                Err(TokenNotExists)
            );
        }

        #[ink::test]
        fn mint_above_limit_fails() {
            let mut sh34 = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let num_of_mints: u64 = MAX_SUPPLY + 1;

            assert_eq!(sh34.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert_eq!(
                sh34.mint_for(accounts.bob, num_of_mints),
                Err(Custom("CollectionFullOrLocked".into()))
            );
        }

        #[ink::test]
        fn mint_low_value_fails() {
            let mut sh34 = init();
            let accounts = default_accounts();
            set_sender(accounts.bob);
            let num_of_mints = 1;

            assert_eq!(sh34.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128 - 1,
            );
            assert_eq!(
                sh34.mint_for(accounts.bob, num_of_mints),
                Err(Custom("BadMintValue".into()))
            );
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128 - 1,
            );
            assert_eq!(sh34.mint_next(), Err(Custom("BadMintValue".into())));
            assert_eq!(sh34.total_supply(), 0);
        }

        #[ink::test]
        fn token_uri_works() {
            let mut sh34 = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);

            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
            assert!(sh34.mint_next().is_ok());
            assert_eq!(
                sh34.token_uri(1),
                Ok(String::from(BASE_URI.to_owned() + "1.json"))
            );
            // return error if request is for not yet minted token
            assert_eq!(sh34.token_uri(42), Err(TokenNotExists));
        }

        #[ink::test]
        fn owner_is_set() {
            let accounts = default_accounts();
            let sh34 = init();
            assert_eq!(sh34.owner(), accounts.alice);
        }

        #[ink::test]
        fn set_base_uri_works() {
            let accounts = default_accounts();
            const NEW_BASE_URI: &str = "new_uri/";
            let mut sh34 = init();

            set_sender(accounts.alice);
            assert!(sh34.set_base_uri(NEW_BASE_URI.into()).is_ok());
            assert_eq!(
                sh34.get_attribute(Id::U8(0), String::from("baseUri")),
                Some(String::from(NEW_BASE_URI))
            );
            set_sender(accounts.bob);
            assert_eq!(
                sh34.set_base_uri("shallFail".into()),
                Err(Custom("O::CallerIsNotOwner".into()))
            );
        }

        #[ink::test]
        fn check_supply_overflow_ok() {
            let max_supply = u64::MAX - 1;
            let mut sh34 = Shiden34Contract::new(
                String::from("Shiden34"),
                String::from("SH34"),
                String::from(BASE_URI),
                max_supply,
                PRICE,
            );
            sh34.last_token_id = max_supply - 1;

            // check case when last_token_id.add(mint_amount) if more than u64::MAX
            assert_eq!(sh34.check_amount(3), Err(Custom("CollectionFullOrLocked".into())));

            // check case when mint_amount is 0
            assert_eq!(sh34.check_amount(0), Err(Custom("CannotMintZeroTokens".into())));
        }

        #[ink::test]
        fn check_value_overflow_ok() {
            let max_supply = u64::MAX;
            let price = u128::MAX as u128;
            let sh34 = Shiden34Contract::new(
                String::from("Shiden34"),
                String::from("SH34"),
                String::from(BASE_URI),
                max_supply,
                price,
            );
            let transferred_value = u128::MAX;
            let mint_amount = u64::MAX;
            assert_eq!(
                sh34.check_value(transferred_value, mint_amount),
                Err(Custom("BadMintValue".into()))
            );
        }

        fn default_accounts() -> test::DefaultAccounts<ink_env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<Environment>(sender);
        }
    }
}
