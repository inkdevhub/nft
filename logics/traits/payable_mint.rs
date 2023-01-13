use openbrush::{
    contracts::{
        psp34::PSP34Error,
        psp34::extensions::enumerable::*
    },
    traits::{
        AccountId,
    },
};

#[openbrush::wrapper]
pub type PayableMintRef = dyn PayableMint;

#[openbrush::trait_definition]
pub trait PayableMint {
    #[ink(message, payable)]
    fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}