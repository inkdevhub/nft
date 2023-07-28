use openbrush::traits::Balance;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub last_token_id: u64,
    pub max_supply: u64,
    pub price_per_mint: Balance,
}
