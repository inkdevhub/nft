use openbrush::{
    contracts::traits::{
        ownable::*,
        pausable::*,
        psp22::PSP22Error,
    },
    traits::{
        AccountId,
        Balance,
        Timestamp,
    },
};

#[openbrush::trait_definition]
pub trait Pair {
    #[ink(message)]
    fn get_reserves(&self) -> (Balance, Balance, Timestamp);

    /// Only factory (owner) can access this function
    #[ink(message)]
    fn initialize(
        &mut self,
        token_0: AccountId,
        token_1: AccountId,
    ) -> Result<(), PairError>;

    #[ink(message)]
    fn mint(&mut self, to: AccountId) -> Result<Balance, PairError>;

    fn _mint_fee(
        &mut self,
        reserve_0: Balance,
        reserve_1: Balance,
    ) -> Result<bool, PairError>;

    fn _update(
        &mut self,
        balance_0: Balance,
        balance_1: Balance,
        reserve_0: Balance,
        reserve_1: Balance,
    ) -> Result<(), PairError>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PairError {
    PSP22Error(PSP22Error),
    OwnableError(OwnableError),
    PausableError(PausableError),
    InsufficientLiquidityMinted,
    Overflow,
    SubUnderFlow1,
    SubUnderFlow2,
    SubUnderFlow3,
    MulOverFlow1,
    MulOverFlow2,
    MulOverFlow3,
    DivByZero1,
    DivByZero2,
}

impl From<OwnableError> for PairError {
    fn from(error: OwnableError) -> Self {
        PairError::OwnableError(error)
    }
}

impl From<PausableError> for PairError {
    fn from(access: PausableError) -> Self {
        PairError::PausableError(access)
    }
}

impl From<PSP22Error> for PairError {
    fn from(error: PSP22Error) -> Self {
        PairError::PSP22Error(error)
    }
}
