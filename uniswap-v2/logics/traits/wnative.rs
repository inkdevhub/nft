use openbrush::{
    contracts::psp22::PSP22Error,
    traits::Balance,
};

#[openbrush::wrapper]
pub type WnativeRef = dyn Wnative;

#[openbrush::trait_definition]
pub trait Wnative {
    /// @notice Deposit NATIVE to wrap it
    #[ink(message, payable)]
    fn deposit(&mut self) -> Result<(), PSP22Error>;

    /// @notice Unwrap NATIVE
    #[ink(message)]
    fn withdraw(&mut self, amount: Balance) -> Result<(), PSP22Error>;
}
