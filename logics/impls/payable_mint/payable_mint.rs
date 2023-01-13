pub use crate::traits::payable_mint::PayableMint;
use openbrush::{
    contracts::{
        psp34::extensions::{
            enumerable::*,
        },
    },
    traits::{
        AccountId,
        Storage,
        String
    },
};

impl<T> PayableMint for T
where
    T: Storage<psp34::Data<enumerable::Balances>>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
    default fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
        if Self::env().transferred_value() != 1_000_000_000_000_000_000 {
            return Err(PSP34Error::Custom(String::from("BadMintValue")))
        }
        self._mint_to(account, id)
    }
}