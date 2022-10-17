pub use crate::traits::{
    data::{
        Data,
        Pool,
    },
    getters::FarmingGetters,
};
use openbrush::{
    contracts::{
        ownable::*,
        traits::psp22::PSP22Ref,
    },
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
        if lp_tokens.iter().any(|lp| *lp == lp_token) {
            return Err(FarmingError::DuplicateLPToken)
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
        let pool = self
            .get_pool_infos(pool_id)
            .ok_or(FarmingError::PoolNotFound)?;
        let current_block = Self::env().block_number();
        if current_block > pool.last_reward_block {
            let lp_token = self
                .get_lp_token(pool_id)
                .ok_or(FarmingError::LpTokenNotFound)?;
            let lp_supply = PSP22Ref::balance_of(&lp_token, Self::env().account_id());
            if lp_supply > 0 {
                let additional_acc_arsw_per_share =
                    self._calculate_additional_acc_arsw_per_share(pool, current_block, lp_supply)?;
            }
        }
        Ok(())
    }

    fn _calculate_additional_acc_arsw_per_share(
        &mut self,
        pool_info: Pool,
        current_block: u32,
        lp_supply: Balance,
    ) -> Result<Balance, FarmingError> {
        if lp_supply == 0 {
            return Err(FarmingError::LpSupplyIsZero)
        }
        let last_reward_block_period = self._get_period(pool_info.last_reward_block)?;
        let current_period = self._get_period(Self::env().block_number())?;
        Ok(10u128)
    }

    fn _get_period(&self, block_number: u32) -> Result<u32, FarmingError> {
        if block_number < ARTHSWAP_ORIGIN_BLOCK {
            return Err(FarmingError::BlockNumberLowerThanOriginBlock)
        }

        // BLOCK_PER_PERIOD is never 0
        return Ok(block_number
            .checked_sub(ARTHSWAP_ORIGIN_BLOCK)
            .ok_or(FarmingError::SubUnderflow1)?
            / BLOCK_PER_PERIOD)
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FarmingError {
    OwnableError(OwnableError),
    DuplicateLPToken,
    PoolNotFound,
    LpTokenNotFound,
    LpSupplyIsZero,
    BlockNumberLowerThanOriginBlock,
    SubUnderflow1,
}

impl From<OwnableError> for FarmingError {
    fn from(error: OwnableError) -> Self {
        FarmingError::OwnableError(error)
    }
}
