#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod factory {
    use ink_lang::{
        codegen::{
            EmitEvent,
            Env,
        },
        reflect::ContractEventBase,
        ToAccountId,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::{
        Storage,
        ZERO_ADDRESS,
    };
    use pair_contract::pair::PairContractRef;
    use uniswap_v2::{
        impls::factory::*,
        traits::factory::*,
    };
    type BaseEvent = <FactoryContract as ContractEventBase>::Type;

    #[ink(event)]
    pub struct PairCreated {
        #[ink(topic)]
        pub token_0: AccountId,
        #[ink(topic)]
        pub token_1: AccountId,
        pub pair: AccountId,
        pub pair_len: u64,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct FactoryContract {
        #[storage_field]
        factory: data::Data,
    }

    impl Factory for FactoryContract {
        fn _instantiate_pair(&mut self, salt_bytes: &[u8]) -> AccountId {
            let pair_hash = self.factory.pair_contract_code_hash;
            let pair = PairContractRef::new()
                .endowment(0)
                .code_hash(pair_hash)
                .salt_bytes(salt_bytes)
                .instantiate()
                .unwrap();
            pair.to_account_id()
        }
    }

    impl Internal for FactoryContract {
        fn _emit_create_pair_event(
            &self,
            _token_0: AccountId,
            _token_1: AccountId,
            _pair: AccountId,
            _pair_len: u64,
        ) {
            // TODO fix compilation
            // self.env().emit_event(PairCreated {
            //     token_0,
            //     token_1,
            //     pair,
            //     pair_len,
            // })
        }
    }

    impl FactoryContract {
        #[ink(constructor)]
        pub fn new(fee_to_setter: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.factory.fee_to_setter = fee_to_setter;
                instance.factory.fee_to = ZERO_ADDRESS.into();
            })
        }
    }
}
