use openbrush::traits::DefaultEnv;
use openbrush::{
    contracts::psp34::*,
    traits::{AccountId, String},
};

#[openbrush::trait_definition]
pub trait PayableMintImpl: psp34::InternalImpl {
    #[ink(message, payable)]
    fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
        if Self::env().transferred_value() != 1_000_000_000_000_000_000 {
            return Err(PSP34Error::Custom(String::from("BadMintValue")));
        }

        psp34::InternalImpl::_mint_to(self, account, id)
    }
}
