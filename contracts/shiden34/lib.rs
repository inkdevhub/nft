#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Enumerable, PSP34Metadata, PSP34Mintable, Ownable)]
#[openbrush::contract]
pub mod shiden34 {
    use ink::codegen::{EmitEvent, Env};
    use openbrush::traits::Storage;
    use payable_mint_pkg::impls::payable_mint::*;
    use payable_mint_pkg::traits::payable_mint::*;

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

    // Override event emission methods
    #[overrider(psp34::Internal)]
    fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
        self.env().emit_event(Transfer { from, to, id });
    }

    #[overrider(psp34::Internal)]
    fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool) {
        self.env().emit_event(Approval {
            from,
            to,
            id,
            approved,
        });
    }

    impl payable_mint::Internal for Shiden34 {}

    impl payable_mint::PayableMintImpl for Shiden34 {}

    impl PayableMint for Shiden34 {
        #[ink(message, payable)]
        fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error> {
            payable_mint::PayableMintImpl::mint(self, to, mint_amount)
        }

        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), PSP34Error> {
            payable_mint::PayableMintImpl::set_base_uri(self, uri)
        }

        #[ink(message)]
        fn token_uri(&self, token_id: u64) -> Result<String, PSP34Error> {
            payable_mint::PayableMintImpl::token_uri(self, token_id)
        }

        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        fn withdraw(&mut self) -> Result<(), PSP34Error> {
            payable_mint::PayableMintImpl::withdraw(self)
        }

        #[ink(message)]
        fn max_supply(&self) -> u64 {
            payable_mint::PayableMintImpl::max_supply(self)
        }

        #[ink(message)]
        fn price(&self) -> Balance {
            payable_mint::PayableMintImpl::price(self)
        }
    }

    impl Shiden34 {
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            base_uri: String,
            max_supply: u64,
            price_per_mint: Balance,
        ) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            ownable::InternalImpl::_init_with_owner(&mut instance, caller);
            let collection_id = psp34::PSP34Impl::collection_id(&instance);
            metadata::InternalImpl::_set_attribute(
                &mut instance,
                collection_id.clone(),
                String::from("name"),
                name,
            );
            metadata::InternalImpl::_set_attribute(
                &mut instance,
                collection_id.clone(),
                String::from("symbol"),
                symbol,
            );
            metadata::InternalImpl::_set_attribute(
                &mut instance,
                collection_id,
                String::from("baseUri"),
                base_uri,
            );
            instance.payable_mint.max_supply = max_supply;
            instance.payable_mint.price_per_mint = price_per_mint;
            instance.payable_mint.last_token_id = 0;
            instance
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::shiden34::PSP34Error::*;
        use ink::env::test;

        const PRICE: Balance = 100_000_000_000_000_000;

        fn init() -> Shiden34 {
            const BASE_URI: &str = "ipfs://myIpfsUri/";
            const MAX_SUPPLY: u64 = 10;
            Shiden34::new(
                String::from("Shiden34"),
                String::from("SH34"),
                String::from(BASE_URI),
                MAX_SUPPLY,
                PRICE,
            )
        }

        #[ink::test]
        fn mint_multiple_works() {
            let mut sh34 = init();
            let accounts = test::default_accounts::<Environment>();
            set_sender(accounts.bob);
            let num_of_mints: u64 = 5;

            assert_eq!(PSP34Impl::total_supply(&sh34), 0);
            test::set_value_transferred::<ink::env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert!(
                payable_mint::PayableMintImpl::mint(&mut sh34, accounts.bob, num_of_mints).is_ok()
            );
            assert_eq!(PSP34Impl::total_supply(&sh34), num_of_mints as u128);
            assert_eq!(PSP34Impl::balance_of(&sh34, accounts.bob), 5);
            assert_eq!(
                PSP34EnumerableImpl::owners_token_by_index(&sh34, accounts.bob, 0),
                Ok(Id::U64(1))
            );
            assert_eq!(
                PSP34EnumerableImpl::owners_token_by_index(&sh34, accounts.bob, 1),
                Ok(Id::U64(2))
            );
            assert_eq!(
                PSP34EnumerableImpl::owners_token_by_index(&sh34, accounts.bob, 2),
                Ok(Id::U64(3))
            );
            assert_eq!(
                PSP34EnumerableImpl::owners_token_by_index(&sh34, accounts.bob, 3),
                Ok(Id::U64(4))
            );
            assert_eq!(
                PSP34EnumerableImpl::owners_token_by_index(&sh34, accounts.bob, 4),
                Ok(Id::U64(5))
            );
            assert_eq!(
                PSP34EnumerableImpl::owners_token_by_index(&sh34, accounts.bob, 5),
                Err(TokenNotExists)
            );
            assert_eq!(5, ink::env::test::recorded_events().count());
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<Environment>(sender);
        }
    }
}
