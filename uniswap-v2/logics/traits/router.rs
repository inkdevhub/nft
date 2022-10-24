use crate::traits::{
    factory::FactoryError,
    pair::PairError,
};
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::traits::psp22::PSP22Error,
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
        reserve_b: Balance,
    ) -> Result<Balance, RouterError>;

    #[ink(message)]
    fn get_amount_out(
        &self,
        amount_in: Balance,
        reserve_in: Balance,
        reserve_out: Balance,
    ) -> Result<Balance, RouterError>;

    #[ink(message)]
    fn get_amount_in(
        &self,
        amount_out: Balance,
        reserve_in: Balance,
        reserve_out: Balance,
    ) -> Result<Balance, RouterError>;

    #[ink(message)]
    fn get_amounts_out(
        &self,
        amount_in: Balance,
        path: Vec<AccountId>,
    ) -> Result<Vec<Balance>, RouterError>;

    #[ink(message)]
    fn get_amounts_in(
        &self,
        amount_out: Balance,
        path: Vec<AccountId>,
    ) -> Result<Vec<Balance>, RouterError>;

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
        deadline: u64,
    ) -> Result<(Balance, Balance, Balance), RouterError>;

    #[ink(message)]
    fn remove_liquidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        liquidity: Balance,
        amount_a_min: Balance,
        amount_b_min: Balance,
        to: AccountId,
        deadline: u64,
    ) -> Result<(Balance, Balance), RouterError>;

    #[ink(message)]
    fn swap_exact_tokens_for_tokens(
        &mut self,
        amount_in: Balance,
        amount_out_min: Balance,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u64,
    ) -> Result<Vec<Balance>, RouterError>;

    #[ink(message)]
    fn swap_tokens_for_exact_tokens(
        &mut self,
        amount_out: Balance,
        amount_in_max: Balance,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u64,
    ) -> Result<Vec<Balance>, RouterError>;

    fn _quote(
        &self,
        amount_a: Balance,
        reserve_a: Balance,
        reserve_b: Balance,
    ) -> Result<Balance, RouterError>;

    fn _get_amount_out(
        &self,
        amount_in: Balance,
        reserve_in: Balance,
        reserve_out: Balance,
    ) -> Result<Balance, RouterError>;

    fn _get_amount_in(
        &self,
        amount_out: Balance,
        reserve_in: Balance,
        reserve_out: Balance,
    ) -> Result<Balance, RouterError>;

    fn _get_amounts_out(
        &self,
        factory: AccountId,
        amount_in: Balance,
        path: &Vec<AccountId>,
    ) -> Result<Vec<Balance>, RouterError>;

    fn _get_amounts_in(
        &self,
        factory: AccountId,
        amount_out: Balance,
        path: &Vec<AccountId>,
    ) -> Result<Vec<Balance>, RouterError>;

    fn _add_liquidity(
        &self,
        token_a: AccountId,
        token_b: AccountId,
        amount_a_desired: Balance,
        amount_b_desired: Balance,
        amount_a_min: Balance,
        amount_b_min: Balance,
    ) -> Result<(Balance, Balance), RouterError>;

    fn _swap(
        &mut self,
        amounts: Vec<Balance>,
        path: Vec<AccountId>,
        to: AccountId,
    ) -> Result<(), RouterError>;

    fn _pair_for(
        &self,
        factory: AccountId,
        token_a: AccountId,
        token_b: AccountId,
    ) -> Result<AccountId, RouterError>;

    fn _sort_tokens(
        &self,
        token_a: AccountId,
        token_b: AccountId,
    ) -> Result<(AccountId, AccountId), RouterError>;

    fn _get_reserves(
        &self,
        factory: AccountId,
        token_a: AccountId,
        token_b: AccountId,
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
    ExcessiveAAmount,
    InsufficientLiquidity,
    InsufficientOutputAmount,
    ExcessiveInputAmount,
    InvalidPath,
    ZeroAddress,
    IdenticalAddresses,
    Expired,
    AddOverFlow1,
    AddOverFlow2,
    SubUnderFlow1,
    MulOverFlow1,
    MulOverFlow2,
    DivByZero1,
    DivByZero2,
    DivByZero3,
    CastOverflow1,
    CastOverflow2,
    CastOverflow3,
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
