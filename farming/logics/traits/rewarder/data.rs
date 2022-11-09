use ink_env::AccountId;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub reward_multiplier: u32,
    pub reward_token: AccountId,
    pub master_chef: AccountId,
}
