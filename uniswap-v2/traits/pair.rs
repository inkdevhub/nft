use openbrush::traits::{AccountId, Balance, Timestamp};

#[openbrush::trait_definition]
pub trait Pair {
    #[ink(message)]
    fn get_reserves(&self) -> (Balance, Balance, Timestamp);

    #[ink(message)]
    fn initialize(&mut self, token_0: AccountId, token_1: AccountId);
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PairError {
    Error
}