pub use crate::traits::data::Data;
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
};

pub const ACC_ARSW_PRECISION: u8 = 12;
pub const ARTHSWAP_ORIGIN_BLOCK: u32 = 1u32;
pub const BLOCK_PER_PERIOD: u32 = 215000u32;
pub const MAX_PERIOD: u8 = 23u8;
pub const FIRST_PERIOD_REWERD_SUPPLY: Balance = 151629858171523000000u128;

#[openbrush::trait_definition]
pub trait Farming: Storage<Data> {
    #[ink(message)]
    fn check_pool_duplicate(&self, lp_token: AccountId) -> Result<(), FarmingError> {
        let lp_tokens = &self.data::<Data>().lp_tokens;
        for i in 0..lp_tokens.len() {
            if lp_tokens[i] == lp_token {
                return Err(FarmingError::DuplicateLPToken)
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FarmingError {
    DuplicateLPToken,
}
