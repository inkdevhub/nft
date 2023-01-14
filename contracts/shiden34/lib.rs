#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod shiden34 {
    // imports from ink!
	use ink_storage::traits::SpreadAllocate;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    // imports from openbrush
	use openbrush::traits::String;
	use openbrush::traits::Storage;
	use openbrush::contracts::ownable::*;
	use openbrush::contracts::psp34::extensions::enumerable::*;
	use openbrush::contracts::psp34::extensions::metadata::*;

	use pallet_payable_mint::{
		traits::payable_mint::*,
		impls::payable_mint::*,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
    	#[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
		#[storage_field]
		ownable: ownable::Data,
		#[storage_field]
		metadata: metadata::Data,
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

    // Section contains default implementation without any modifications
	impl PSP34 for Contract {}
	impl Ownable for Contract {}

	impl PSP34Enumerable for Contract {}
	impl PSP34Metadata for Contract {}
    impl PayableMint for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(
			name: String,
            symbol: String,
            base_uri: String,
            max_supply: u64,
            price_per_mint: Balance,
		) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Contract|{
				instance._init_with_owner(instance.env().caller());
				let collection_id = instance.collection_id();
                instance._set_attribute(collection_id.clone(), String::from("name"), name);
                instance._set_attribute(collection_id.clone(), String::from("symbol"), symbol);
                instance._set_attribute(collection_id, String::from("baseUri"), base_uri);
				instance.payable_mint.max_supply = max_supply;
                instance.payable_mint.price_per_mint = price_per_mint;
                instance.payable_mint.last_token_id = 0;
			})
        }
    }

	// Override event emission methods
	impl psp34::Internal for Contract {
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

	#[cfg(test)]
    mod tests {
        use super::*;
        use crate::shiden34::PSP34Error::*;
        use ink_env::test;
        use ink_lang as ink;

        const PRICE: Balance = 100_000_000_000_000_000;
		
        fn init() -> Contract {
			const BASE_URI: &str = "ipfs://myIpfsUri/";
			const MAX_SUPPLY: u64 = 10;
            Contract::new(
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

            assert_eq!(sh34.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert!(sh34.mint(accounts.bob, num_of_mints).is_ok());
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


        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<Environment>(sender);
        }
	}
}