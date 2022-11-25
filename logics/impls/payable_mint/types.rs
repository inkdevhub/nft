use openbrush::traits::{
    Balance,
    String,
};
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub last_token_id: u64,
    pub collection_id: u32,
    pub max_supply: u64,
    pub price_per_mint: Balance,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Shiden34Error {
    CannotMintZeroTokens,
    CollectionIsFull,
    BadMintValue,
    WithdrawalFailed,
}

impl Shiden34Error {
    pub fn as_str(&self) -> String {
        match self {
            Shiden34Error::CannotMintZeroTokens => String::from("CannotMintZeroTokens"),
            Shiden34Error::CollectionIsFull => String::from("CollectionIsFull"),
            Shiden34Error::BadMintValue => String::from("BadMintValue"),
            Shiden34Error::WithdrawalFailed => String::from("WithdrawalFailed"),
        }
    }
}
