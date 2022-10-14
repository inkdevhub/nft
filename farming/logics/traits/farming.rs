pub use crate::traits::{
    data::Data,
    getters::FarmingGetters,
};
use openbrush::{
    contracts::ownable::*,
    modifiers,
    traits::{
        AccountId,
        Balance,
        Storage,
    },
};

pub const ACC_ARSW_PRECISION: u8 = 12;
pub const ARTHSWAP_ORIGIN_BLOCK: u32 = 1u32;
pub const BLOCK_PER_PERIOD: u32 = 215000u32;
pub const MAX_PERIOD: u8 = 23u8;
pub const FIRST_PERIOD_REWERD_SUPPLY: Balance = 151629858171523000000u128;

#[openbrush::trait_definition]
pub trait Farming: Storage<Data> + Storage<ownable::Data> + FarmingGetters {
    #[ink(message)]
    #[modifiers(only_owner)]
    fn add(
        &mut self,
        alloc_point: u128,
        lp_token: AccountId,
        rewarder: AccountId,
    ) -> Result<(), FarmingError> {
        self._check_pool_duplicate(lp_token)?;
        self._update_all_pools()?;
        Ok(())
    }

    fn _check_pool_duplicate(&self, lp_token: AccountId) -> Result<(), FarmingError> {
        let lp_tokens = &self.data::<Data>().lp_tokens;
        for i in 0..lp_tokens.len() {
            if lp_tokens[i] == lp_token {
                return Err(FarmingError::DuplicateLPToken)
            }
        }
        Ok(())
    }

    fn _update_all_pools(&mut self) -> Result<(), FarmingError> {
        let lp_tokens = &self.data::<Data>().lp_tokens;
        for i in 0..lp_tokens.len() {
            self._update_pool(i as u32)?;
        }
        Ok(())
    }

    fn _update_pool(&mut self, pool_id: u32) -> Result<(), FarmingError> {
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FarmingError {
    OwnableError(OwnableError),
    DuplicateLPToken,
}

impl From<OwnableError> for FarmingError {
    fn from(error: OwnableError) -> Self {
        FarmingError::OwnableError(error)
    }
}
