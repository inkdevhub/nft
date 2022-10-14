use openbrush::traits::{
    AccountId,
    Balance,
    Timestamp,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub arsw_token: AccountId,
}

#[openbrush::trait_definition]
pub trait Farming {
    #[ink(message)]
    fn add(&self) -> u32 {
        10u32
    }
}
