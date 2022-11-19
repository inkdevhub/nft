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
pub enum ShidenGraffitiError {
    CannotMintZeroTokens,
    CollectionIsFull,
    BadMintValue,
    WithdrawalFailed,
}

impl ShidenGraffitiError {
    pub fn as_str(&self) -> String {
        match self {
            ShidenGraffitiError::CannotMintZeroTokens => String::from("CannotMintZeroTokens"),
            ShidenGraffitiError::CollectionIsFull => String::from("CollectionIsFull"),
            ShidenGraffitiError::BadMintValue => String::from("BadMintValue"),
            ShidenGraffitiError::WithdrawalFailed => String::from("WithdrawalFailed"),
        }
    }
}
