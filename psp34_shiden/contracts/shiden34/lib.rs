#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod shiden_graffiti {
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::extensions::{
                enumerable::*,
                metadata::*,
            },
            reentrancy_guard::*,
        },
        traits::{
            Storage,
            String,
        },
    };

    use psp34_helper::{
        impls::psp34_custom::*,
        traits::psp34_custom::*,
    };

    // ShidenGraffiti contract storage
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ShidenGraffitiContract {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        psp34_custom: psp34_custom_types::Data,
    }

    impl PSP34 for ShidenGraffitiContract {}
    impl PSP34Enumerable for ShidenGraffitiContract {}
    impl PSP34Metadata for ShidenGraffitiContract {}
    impl Ownable for ShidenGraffitiContract {}

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

    impl ShidenGraffitiContract {
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            base_uri: String,
            max_supply: u64,
            price_per_mint: Balance,
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut ShidenGraffitiContract| {
                let collection_id = instance.collection_id();
                instance._set_attribute(collection_id.clone(), String::from("name"), name);
                instance._set_attribute(collection_id.clone(), String::from("symbol"), symbol);
                instance._set_attribute(collection_id, String::from("baseUri"), base_uri);
                instance.psp34_custom.max_supply = max_supply;
                instance.psp34_custom.price_per_mint = price_per_mint;
                instance.psp34_custom.last_token_id = 0;
                let caller = instance.env().caller();
                instance._init_with_owner(caller);
            })
        }
    }

    // Override event emission methods
    impl psp34_custom::Internal for ShidenGraffitiContract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

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
        }
    }

    impl Psp34Custom for ShidenGraffitiContract {}

    // ------------------- T E S T -----------------------------------------------------
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::shiden_graffiti::PSP34Error::*;
        use ink_env::{
            pay_with_call,
            test,
        };
        use ink_lang as ink;
        use ink_prelude::string::String as PreludeString;
        use psp34_helper::impls::psp34_custom::{
            psp34_custom::Internal,
            psp34_custom_types::ShidenGraffitiError,
        };
        const PRICE: Balance = 100_000_000_000_000_000;
        const BASE_URI: &str = "ipfs://myIpfsUri/";
        const MAX_SUPPLY: u64 = 10;

        #[ink::test]
        fn init_works() {
            let shg = init();
            let collection_id = shg.collection_id();
            assert_eq!(
                shg.get_attribute(collection_id.clone(), String::from("name")),
                Some(String::from("ShidenGraffiti"))
            );
            assert_eq!(
                shg.get_attribute(collection_id.clone(), String::from("symbol")),
                Some(String::from("SH34"))
            );
            assert_eq!(
                shg.get_attribute(collection_id, String::from("baseUri")),
                Some(String::from(BASE_URI))
            );
            assert_eq!(shg.max_supply(), MAX_SUPPLY);
            assert_eq!(shg.price(), PRICE);
        }

        fn init() -> ShidenGraffitiContract {
            ShidenGraffitiContract::new(
                String::from("ShidenGraffiti"),
                String::from("SH34"),
                String::from(BASE_URI),
                MAX_SUPPLY,
                PRICE,
            )
        }

        #[ink::test]
        fn mint_single_works() {
            let mut shg = init();
            let accounts = default_accounts();
            assert_eq!(shg.owner(), accounts.alice);
            set_sender(accounts.bob);

            assert_eq!(shg.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
            assert!(shg.mint_next().is_ok());
            assert_eq!(shg.total_supply(), 1);
            assert_eq!(shg.owner_of(Id::U64(1)), Some(accounts.bob));
            assert_eq!(shg.balance_of(accounts.bob), 1);

            assert_eq!(shg.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            assert_eq!(shg.psp34_custom.last_token_id, 1);
            assert_eq!(1, ink_env::test::recorded_events().count());
        }

        #[ink::test]
        fn mint_multiple_works() {
            let mut shg = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let num_of_mints: u64 = 5;

            assert_eq!(shg.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert!(shg.mint_for(accounts.bob, num_of_mints).is_ok());
            assert_eq!(shg.total_supply(), num_of_mints as u128);
            assert_eq!(shg.balance_of(accounts.bob), 5);
            assert_eq!(shg.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            assert_eq!(shg.owners_token_by_index(accounts.bob, 1), Ok(Id::U64(2)));
            assert_eq!(shg.owners_token_by_index(accounts.bob, 2), Ok(Id::U64(3)));
            assert_eq!(shg.owners_token_by_index(accounts.bob, 3), Ok(Id::U64(4)));
            assert_eq!(shg.owners_token_by_index(accounts.bob, 4), Ok(Id::U64(5)));
            assert_eq!(5, ink_env::test::recorded_events().count());
            assert_eq!(
                shg.owners_token_by_index(accounts.bob, 5),
                Err(TokenNotExists)
            );
        }

        #[ink::test]
        fn mint_above_limit_fails() {
            let mut shg = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let num_of_mints: u64 = MAX_SUPPLY + 1;

            assert_eq!(shg.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert_eq!(
                shg.mint_for(accounts.bob, num_of_mints),
                Err(PSP34Error::Custom(
                    ShidenGraffitiError::CollectionIsFull.as_str()
                ))
            );
        }

        #[ink::test]
        fn mint_low_value_fails() {
            let mut shg = init();
            let accounts = default_accounts();
            set_sender(accounts.bob);
            let num_of_mints = 1;

            assert_eq!(shg.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128 - 1,
            );
            assert_eq!(
                shg.mint_for(accounts.bob, num_of_mints),
                Err(PSP34Error::Custom(
                    ShidenGraffitiError::BadMintValue.as_str()
                ))
            );
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128 - 1,
            );
            assert_eq!(
                shg.mint_next(),
                Err(PSP34Error::Custom(
                    ShidenGraffitiError::BadMintValue.as_str()
                ))
            );
            assert_eq!(shg.total_supply(), 0);
        }

        #[ink::test]
        fn withdrawal_works() {
            let mut shg = init();
            let accounts = default_accounts();
            set_balance(accounts.bob, PRICE);
            set_sender(accounts.bob);

            assert!(pay_with_call!(shg.mint_next(), PRICE).is_ok());
            let expected_contract_balance = PRICE + shg.env().minimum_balance();
            assert_eq!(shg.env().balance(), expected_contract_balance);

            // Bob fails to withdraw
            set_sender(accounts.bob);
            assert!(shg.withdraw().is_err());
            assert_eq!(shg.env().balance(), expected_contract_balance);

            // Alice (contract owner) withdraws. Existential minimum is still set
            set_sender(accounts.alice);
            assert!(shg.withdraw().is_ok());
            // assert_eq!(shg.env().balance(), shg.env().minimum_balance());
        }

        #[ink::test]
        fn token_uri_works() {
            let mut shg = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);

            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
            assert!(shg.mint_next().is_ok());
            // return error if request is for not yet minted token
            assert_eq!(shg.token_uri(42), Err(TokenNotExists));
            assert_eq!(
                shg.token_uri(1),
                Ok(PreludeString::from(BASE_URI.to_owned() + "1.json"))
            );

            // return error if request is for not yet minted token
            assert_eq!(shg.token_uri(42), Err(TokenNotExists));

            // verify token_uri when baseUri is empty
            set_sender(accounts.alice);
            assert!(shg.set_base_uri(PreludeString::from("")).is_ok());
            assert_eq!(
                shg.token_uri(1),
                Ok("".to_owned() + &PreludeString::from("1.json"))
            );
        }

        #[ink::test]
        fn owner_is_set() {
            let accounts = default_accounts();
            let shg = init();
            assert_eq!(shg.owner(), accounts.alice);
        }

        #[ink::test]
        fn set_base_uri_works() {
            let accounts = default_accounts();
            const NEW_BASE_URI: &str = "new_uri/";
            let mut shg = init();

            set_sender(accounts.alice);
            let collection_id = shg.collection_id();
            assert!(shg.set_base_uri(NEW_BASE_URI.into()).is_ok());
            assert_eq!(
                shg.get_attribute(collection_id, String::from("baseUri")),
                Some(String::from(NEW_BASE_URI))
            );
            set_sender(accounts.bob);
            assert_eq!(
                shg.set_base_uri(NEW_BASE_URI.into()),
                Err(PSP34Error::Custom(String::from("O::CallerIsNotOwner")))
            );
        }

        #[ink::test]
        fn check_supply_overflow_ok() {
            let max_supply = u64::MAX - 1;
            let mut shg = ShidenGraffitiContract::new(
                String::from("ShidenGraffiti"),
                String::from("SH34"),
                String::from(BASE_URI),
                max_supply,
                PRICE,
            );
            shg.psp34_custom.last_token_id = max_supply - 1;

            // check case when last_token_id.add(mint_amount) if more than u64::MAX
            assert_eq!(
                shg._check_amount(3),
                Err(PSP34Error::Custom(
                    ShidenGraffitiError::CollectionIsFull.as_str()
                ))
            );

            // check case when mint_amount is 0
            assert_eq!(
                shg._check_amount(0),
                Err(PSP34Error::Custom(
                    ShidenGraffitiError::CannotMintZeroTokens.as_str()
                ))
            );
        }

        #[ink::test]
        fn check_value_overflow_ok() {
            let max_supply = u64::MAX;
            let price = u128::MAX as u128;
            let shg = ShidenGraffitiContract::new(
                String::from("ShidenGraffiti"),
                String::from("SH34"),
                String::from(BASE_URI),
                max_supply,
                price,
            );
            let transferred_value = u128::MAX;
            let mint_amount = u64::MAX;
            assert_eq!(
                shg._check_value(transferred_value, mint_amount),
                Err(PSP34Error::Custom(
                    ShidenGraffitiError::BadMintValue.as_str()
                ))
            );
        }

        fn default_accounts() -> test::DefaultAccounts<ink_env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<Environment>(sender);
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(account_id, balance)
        }
    }
}
