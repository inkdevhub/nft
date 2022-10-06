use crate::traits::{
    pair::PairError,
    factory::FactoryError
};
use openbrush::{
    contracts::{
        traits::{
            psp22::PSP22Error,
        },
    },
    traits::{
        AccountId,
        Balance,
    },
};

#[openbrush::wrapper]
pub type RouterRef = dyn Router;

#[openbrush::trait_definition]
pub trait Router {
    #[ink(message)]
    fn factory(&self) -> AccountId;

    #[ink(message)]
    fn quote(
        &self,
        amount_a: Balance,
        reserve_a: Balance,
        reserve_b: Balance
    ) -> Result<Balance, RouterError>;

    #[ink(message)]
    fn get_amount_out(
        &self,
        amount_in: Balance,
        reserve_in: Balance,
        reserve_out: Balance
    ) -> Result<Balance, RouterError>;

    #[ink(message)]
    fn get_amount_in(
        &self,
        amount_out: Balance,
        reserve_in: Balance,
        reserve_out: Balance
    ) -> Result<Balance, RouterError>;

    #[ink(message)]
    fn add_liquidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        amount_a_desired: Balance,
        amount_b_desired: Balance,
        amount_a_min: Balance,
        amount_b_min: Balance,
        to: AccountId,
        dead_line: u64,
    ) ->  Result<(Balance, Balance, Balance), RouterError>;

    #[ink(message)]
    fn remove_lequidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        liquidity: Balance,
        amount_a_min: Balance,
        amount_b_min: Balance,
        to: AccountId,
        dead_line: u64,
    ) -> Result<(Balance, Balance), RouterError>;

    fn _quote(
        &self,
        amount_a: Balance,
        reserve_a: Balance,
        reserve_b: Balance
    ) -> Result<Balance, RouterError>;

    fn _get_amount_out(
        &self,
        amount_in: Balance,
        reserve_in: Balance,
        reserve_out: Balance
    ) -> Result<Balance, RouterError>;

    fn _get_amount_in(
        &self,
        amount_out: Balance,
        reserve_in: Balance,
        reserve_out: Balance
    ) -> Result<Balance, RouterError>;

    fn _add_liquidity(
        &self,
        token_a: AccountId,
        token_b: AccountId,
        amount_a_desired: Balance,
        amount_b_desired: Balance,
        amount_a_min: Balance,
        amount_b_min: Balance,
    ) -> Result<(Balance, Balance), RouterError>;

    fn _pair_for(
        &self,
        factory: AccountId,
        token_a: AccountId,
        token_b: AccountId,
    ) ->  Result<AccountId, RouterError>;

    fn _sort_tokens(
        &self,
        token_a: AccountId,
        token_b: AccountId,
    ) -> Result<(AccountId, AccountId), RouterError>;

    fn _get_reserves(
        &self,
        factory: AccountId,
        token_a: AccountId,
        token_b: AccountId
    ) -> Result<(Balance, Balance), RouterError>;
}

// Error Definition
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RouterError {
    PSP22Error(PSP22Error),
    FactoryError(FactoryError),
    PairError(PairError),
    PairNotFound,
    InsufficientAmount,
    InsufficientAAmount,
    InsufficientBAmount,
    InsufficientLiquidity,
    ZeroAddress,
    IdenticalAddresses,
    Expired,
    AddOverFlow,
    SubUnderFlow,
    MulOverFlow,
    DivByZero,
}

impl From<PSP22Error> for RouterError {
    fn from(error: PSP22Error) -> Self {
        RouterError::PSP22Error(error)
    }
}

impl From<FactoryError> for RouterError {
    fn from(error: FactoryError) -> Self {
        RouterError::FactoryError(error)
    }
}

impl From<PairError> for RouterError {
    fn from(error: PairError) -> Self {
        RouterError::PairError(error)
    }
}
