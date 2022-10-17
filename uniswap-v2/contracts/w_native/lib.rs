#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod w_native_token {
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22::extensions::metadata::*,
        traits::Storage,
    };

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct WNative {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP22 for WNative {}

    impl psp22::Internal for WNative {
        fn _emit_transfer_event(
            &self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            amount: Balance,
        ) {
            self.env().emit_event(Transfer {
                from,
                to,
                value: amount,
            });
        }

        fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
            self.env().emit_event(Approval {
                owner,
                spender,
                value: amount,
            });
        }
    }

    impl PSP22Metadata for WNative {}

    impl WNative {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut WNative| {
                instance.metadata.name = Some(String::from("WNATIVE TOKEN"));
                instance.metadata.symbol = Some(String::from("WNT"));
                instance.metadata.decimals = 18;
            })
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            let transferred_balance = self.env().transferred_value();
            self._mint(caller, transferred_balance)?;
            Ok(())
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            self._burn_from(caller, amount)?;
            self.env()
                .transfer(caller, amount)
                .map_err(|_| PSP22Error::Custom(String::from("Transfer failed")))?;

            Ok(())
        }
    }
}
