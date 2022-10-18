pub use crate::traits::{
    data::{
        Data,
        Pool,
    },
    events::FarmingEvents,
    getters::FarmingGetters,
};
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::{
        ownable::*,
        traits::psp22::{
            PSP22Error,
            PSP22Ref,
        },
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
pub trait Farming: Storage<Data> + Storage<ownable::Data> + FarmingGetters + FarmingEvents {
    #[ink(message)]
    #[modifiers(only_owner)]
    fn add(
        &mut self,
        alloc_point: u32,
        lp_token: AccountId,
        rewarder: Option<AccountId>,
    ) -> Result<(), FarmingError> {
        self._check_pool_duplicate(lp_token)?;
        self._update_all_pools()?;
        self.data::<Data>().total_alloc_point = self
            .data::<Data>()
            .total_alloc_point
            .checked_add(alloc_point)
            .ok_or(FarmingError::AddOverflow2)?;
        self.data::<Data>().lp_tokens.push(lp_token);
        self.data::<Data>().rewarders.push(rewarder);
        let pool_length = self.pool_length();

        self.data::<Data>().pool_info.insert(
            &pool_length,
            &Pool {
                acc_arsw_per_share: 0,
                last_reward_block: Self::env().block_number(),
                alloc_point,
            },
        );
        self.data::<Data>().pool_info_length = pool_length
            .checked_add(1)
            .ok_or(FarmingError::AddOverflow2)?;
        self._emit_log_pool_addition_event(pool_length, alloc_point, lp_token, rewarder);
        Ok(())
    }

    #[ink(message)]
    fn pending_arsw(&self, _pool_id: u32, _user: AccountId) -> Result<Balance, FarmingError> {
        Ok(1_000_000_000_000_000_000)
    }

    #[ink(message)]
    fn deposit(
        &mut self,
        pool_id: u32,
        amount: Balance,
        to: AccountId,
    ) -> Result<(), FarmingError> {
        let (prev_amount, prev_reward_debt) = self.get_user_info(pool_id, to).unwrap_or((0, 0));
        // TODO: Fix reward_debt
        self.data::<Data>().user_info.insert(
            &(pool_id, to),
            &(
                prev_amount
                    .checked_add(amount)
                    .ok_or(FarmingError::AddOverflow1)?,
                prev_reward_debt,
            ),
        );
        let lp_token = self
            .get_lp_token(pool_id)
            .ok_or(FarmingError::PoolNotFound2)?;
        PSP22Ref::transfer_from(
            &lp_token,
            Self::env().caller(),
            Self::env().account_id(),
            amount,
            Vec::new(),
        )?;
        Ok(())
    }

    #[ink(message)]
    fn withdraw(
        &mut self,
        pool_id: u32,
        amount: Balance,
        to: AccountId,
    ) -> Result<(), FarmingError> {
        if amount == 0 {
            return Err(FarmingError::ZeroWithdrawal)
        }
        let caller = Self::env().caller();
        let (prev_amount, prev_reward_debt) = self
            .get_user_info(pool_id, caller)
            .ok_or(FarmingError::UserNotFound)?;
        // TODO: Fix reward_debt
        self.data::<Data>().user_info.insert(
            &(pool_id, caller),
            &(
                prev_amount
                    .checked_sub(amount)
                    .ok_or(FarmingError::SubUnderflow2)?,
                prev_reward_debt,
            ),
        );
        let lp_token = self
            .get_lp_token(pool_id)
            .ok_or(FarmingError::PoolNotFound3)?;
        PSP22Ref::transfer(&lp_token, to, amount, Vec::new())?;
        Ok(())
    }

    #[ink(message)]
    fn harvest(&mut self, _pool_id: u32, _to: AccountId) -> Result<(), FarmingError> {
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
            .ok_or(FarmingError::PoolNotFound1)?;
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
    PSP22Error(PSP22Error),
    DuplicateLPToken,
    PoolNotFound1,
    PoolNotFound2,
    PoolNotFound3,
    UserNotFound,
    ZeroWithdrawal,
    LpTokenNotFound,
    LpSupplyIsZero,
    BlockNumberLowerThanOriginBlock,
    SubUnderflow1,
    SubUnderflow2,
    AddOverflow1,
    AddOverflow2,
    AddOverflow3,
}

impl From<OwnableError> for FarmingError {
    fn from(error: OwnableError) -> Self {
        FarmingError::OwnableError(error)
    }
}

impl From<PSP22Error> for FarmingError {
    fn from(error: PSP22Error) -> Self {
        FarmingError::PSP22Error(error)
    }
}
