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
    }

    #[overrider(PSP34Mintable)]
    #[openbrush::modifiers(only_owner)]
    fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
        psp34::InternalImpl::_mint_to(self, account, id)
    }

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
