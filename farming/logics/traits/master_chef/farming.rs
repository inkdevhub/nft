pub use crate::traits::{
    master_chef::{
        data::{
            Data,
            Pool,
        },
        events::FarmingEvents,
        getters::FarmingGetters,
    },
    rewarder::rewarder::RewarderRef,
};
use crate::{
    ensure,
    helpers::math::casted_mul,
    traits::master_chef::{
        data::UserInfo,
        errors::FarmingError,
    },
};
use ink_env::CallFlags;
use ink_prelude::vec::Vec;
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
use primitive_types::U256;

// Cannot be 0 or it will panic!
pub const ACC_ARSW_PRECISION: u128 = 1_000_000_000_000;
pub const ARTHSWAP_ORIGIN_BLOCK: u32 = 1u32;
pub const BLOCK_PER_PERIOD: u32 = 215000u32;
pub const MAX_PERIOD: u32 = 23u32;
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
            .get_total_alloc_point()
            .checked_add(alloc_point)
            .ok_or(FarmingError::AddOverflow1)?;
        self.data::<Data>().lp_tokens.push(lp_token);
        let pool_length = self.pool_length();

        if let Some(rewarder_address) = rewarder {
            self.data::<Data>()
                .rewarders
                .insert(&pool_length, &rewarder_address);
        }
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
            .ok_or(FarmingError::AddOverflow1)?;

        self._emit_log_pool_addition_event(pool_length, alloc_point, lp_token, rewarder);
        Ok(())
    }

    #[ink(message)]
    #[modifiers(only_owner)]
    fn set(
        &mut self,
        pool_id: u32,
        alloc_point: u32,
        rewarder: Option<AccountId>,
        overwrite: bool,
    ) -> Result<(), FarmingError> {
        let pool_info = self
            .get_pool_info(pool_id)
            .ok_or(FarmingError::PoolNotFound)?;
        self._update_all_pools()?;
        self.data::<Data>().total_alloc_point = self
            .get_total_alloc_point()
            .checked_sub(pool_info.alloc_point)
            .ok_or(FarmingError::SubUnderflow4)?
            .checked_add(alloc_point)
            .ok_or(FarmingError::AddOverflow7)?;

        self.data::<Data>().pool_info.insert(
            &pool_id,
            &Pool {
                alloc_point,
                ..pool_info
            },
        );
        let mut rewarder = rewarder;
        if overwrite {
            match rewarder {
                Some(rewarder_address) => {
                    self.data::<Data>()
                        .rewarders
                        .insert(&pool_id, &rewarder_address)
                }
                None => self.data::<Data>().rewarders.remove(&pool_id),
            }
        } else {
            rewarder = self.get_rewarder(pool_id);
        }
        self._emit_log_set_pool_event(pool_id, alloc_point, rewarder, overwrite);
        Ok(())
    }

    #[ink(message)]
    fn pending_arsw(&self, pool_id: u32, user: AccountId) -> Result<Balance, FarmingError> {
        let pool = self
            .get_pool_info(pool_id)
            .ok_or(FarmingError::PoolNotFound)?;
        let user_info = self
            .get_user_info(pool_id, user)
            .ok_or(FarmingError::UserNotFound)?;
        let mut acc_arsw_per_share = pool.acc_arsw_per_share;

        let lp_supply = self._get_lp_supply(pool_id)?;
        let current_block = Self::env().block_number();

        if current_block > pool.last_reward_block && lp_supply != 0 {
            let additional_acc_arsw_per_share =
                self._calculate_additional_acc_arsw_per_share(&pool, current_block, lp_supply)?;
            acc_arsw_per_share = acc_arsw_per_share
                .checked_add(additional_acc_arsw_per_share)
                .ok_or(FarmingError::AddOverflow6)?;
        }

        let pending = <U256 as TryInto<u128>>::try_into(
            casted_mul(user_info.amount, acc_arsw_per_share) / ACC_ARSW_PRECISION,
        )
        .map_err(|_| FarmingError::CastTou128Error7)?
        .checked_add_signed(-user_info.reward_debt)
        .ok_or(FarmingError::AddOverflow10)?;

        Ok(pending)
    }

    #[ink(message)]
    fn deposit(
        &mut self,
        pool_id: u32,
        amount: Balance,
        to: AccountId,
    ) -> Result<(), FarmingError> {
        let pool = self
            .get_pool_info(pool_id)
            .ok_or(FarmingError::PoolNotFound)?;
        self._update_pool(pool_id)?;
        let user = self.get_user_info(pool_id, to).unwrap_or_default();
        let user_amount = user
            .amount
            .checked_add(amount)
            .ok_or(FarmingError::AddOverflow8)?;
        let user_reward_debt = user
            .reward_debt
            .checked_add(
                <U256 as TryInto<i128>>::try_into(
                    casted_mul(amount, pool.acc_arsw_per_share) / ACC_ARSW_PRECISION,
                )
                .map_err(|_| FarmingError::CastTou128Error1)?,
            )
            .ok_or(FarmingError::AddOverflow9)?;

        self.data::<Data>().user_info.insert(
            &(pool_id, to),
            &UserInfo {
                amount: user_amount,
                reward_debt: user_reward_debt,
            },
        );

        if let Some(rewarder_address) = self.get_rewarder(pool_id) {
            RewarderRef::on_arsw_reward(&rewarder_address, pool_id, to, to, 0, user_amount)?;
        }

        let lp_token = self
            .get_lp_token(pool_id)
            .ok_or(FarmingError::PoolNotFound)?;
        PSP22Ref::transfer_from(
            &lp_token,
            Self::env().caller(),
            Self::env().account_id(),
            amount,
            Vec::new(),
        )?;
        self._emit_deposit_event(Self::env().caller(), pool_id, amount, to);
        Ok(())
    }

    #[ink(message)]
    fn withdraw(
        &mut self,
        pool_id: u32,
        amount: Balance,
        to: AccountId,
    ) -> Result<(), FarmingError> {
        ensure!(amount > 0, FarmingError::ZeroWithdrawal);
        let pool = self
            .get_pool_info(pool_id)
            .ok_or(FarmingError::PoolNotFound)?;
        self._update_pool(pool_id)?;
        let caller = Self::env().caller();
        let user = self
            .get_user_info(pool_id, caller)
            .ok_or(FarmingError::UserNotFound)?;
        let user_reward_debt = user
            .reward_debt
            .checked_sub(
                <U256 as TryInto<i128>>::try_into(
                    casted_mul(amount, pool.acc_arsw_per_share) / ACC_ARSW_PRECISION,
                )
                .map_err(|_| FarmingError::CastTou128Error2)?,
            )
            .ok_or(FarmingError::SubUnderflow5)?;

        let user_amount = user
            .amount
            .checked_sub(amount)
            .ok_or(FarmingError::SubUnderflow8)?;

        self.data::<Data>().user_info.insert(
            &(pool_id, to),
            &UserInfo {
                amount: user_amount,
                reward_debt: user_reward_debt,
            },
        );

        if let Some(rewarder_address) = self.get_rewarder(pool_id) {
            RewarderRef::on_arsw_reward(&rewarder_address, pool_id, caller, to, 0, user_amount)?;
        }

        let lp_token = self
            .get_lp_token(pool_id)
            .ok_or(FarmingError::LpTokenNotFound)?;
        PSP22Ref::transfer(&lp_token, to, amount, Vec::new())?;
        self._emit_withdraw_event(caller, pool_id, amount, to);
        Ok(())
    }

    #[ink(message)]
    fn harvest(&mut self, pool_id: u32, to: AccountId) -> Result<(), FarmingError> {
        let pool = self
            .get_pool_info(pool_id)
            .ok_or(FarmingError::PoolNotFound)?;
        self._update_pool(pool_id)?;
        let caller = Self::env().caller();
        let user = self
            .get_user_info(pool_id, caller)
            .ok_or(FarmingError::UserNotFound)?;

        let accumulated_arsw = <U256 as TryInto<i128>>::try_into(
            casted_mul(user.amount, pool.acc_arsw_per_share) / ACC_ARSW_PRECISION,
        )
        .map_err(|_| FarmingError::CastTou128Error3)?;

        let pending_arsw = accumulated_arsw
            .checked_sub(user.reward_debt)
            .ok_or(FarmingError::SubUnderflow7)? as u128;

        self.data::<Data>().user_info.insert(
            &(pool_id, to),
            &UserInfo {
                reward_debt: accumulated_arsw,
                ..user
            },
        );

        if pending_arsw != 0 {
            PSP22Ref::transfer(
                &mut self.data::<Data>().arsw_token,
                to,
                pending_arsw,
                Vec::new(),
            )?;
        }

        if let Some(rewarder_address) = self.get_rewarder(pool_id) {
            RewarderRef::on_arsw_reward(
                &rewarder_address,
                pool_id,
                caller,
                to,
                pending_arsw,
                user.amount,
            )?;
        }

        self._emit_harvest_event(caller, pool_id, pending_arsw);
        Ok(())
    }

    #[ink(message)]
    fn withdraw_and_harvest(
        &mut self,
        pool_id: u32,
        amount: Balance,
        to: AccountId,
    ) -> Result<(), FarmingError> {
        let pool = self
            .get_pool_info(pool_id)
            .ok_or(FarmingError::PoolNotFound)?;
        self._update_pool(pool_id)?;
        let caller = Self::env().caller();
        let user = self
            .get_user_info(pool_id, caller)
            .ok_or(FarmingError::UserNotFound)?;

        let accumulated_arsw =
            casted_mul(user.amount, pool.acc_arsw_per_share) / ACC_ARSW_PRECISION;

        let pending_arsw = <U256 as TryInto<u128>>::try_into(accumulated_arsw)
            .map_err(|_| FarmingError::CastTou128Error6)?
            .checked_add_signed(-user.reward_debt)
            .ok_or(FarmingError::AddOverflow11)?;

        let user_reward_debt: i128 = accumulated_arsw
            .checked_sub(casted_mul(amount, pool.acc_arsw_per_share) / ACC_ARSW_PRECISION)
            .ok_or(FarmingError::SubUnderflow6)?
            .try_into()
            .map_err(|_| FarmingError::CastTou128Error5)?;

        let user_amount = user
            .amount
            .checked_sub(amount)
            .ok_or(FarmingError::AddOverflow12)?;

        self.data::<Data>().user_info.insert(
            &(pool_id, to),
            &UserInfo {
                reward_debt: user_reward_debt,
                amount: user_amount,
            },
        );

        PSP22Ref::transfer(
            &self.data::<Data>().arsw_token,
            to,
            pending_arsw,
            Vec::new(),
        )?;
        if let Some(rewarder_address) = self.get_rewarder(pool_id) {
            RewarderRef::on_arsw_reward(
                &rewarder_address,
                pool_id,
                caller,
                to,
                pending_arsw,
                user.amount,
            )?;
        }
        let lp_token = self
            .get_lp_token(pool_id)
            .ok_or(FarmingError::LpTokenNotFound)?;
        PSP22Ref::transfer(&lp_token, to, amount, Vec::new())?;

        self._emit_withdraw_event(caller, pool_id, amount, to);
        self._emit_harvest_event(caller, pool_id, pending_arsw);
        Ok(())
    }

    #[ink(message)]
    fn emergency_withdraw(&mut self, pool_id: u32, to: AccountId) -> Result<(), FarmingError> {
        let caller = Self::env().caller();
        let user = self
            .get_user_info(pool_id, caller)
            .ok_or(FarmingError::UserNotFound)?;
        let amount = user.amount;
        self.data::<Data>().user_info.insert(
            &(pool_id, to),
            &UserInfo {
                reward_debt: 0,
                amount: 0,
            },
        );

        if let Some(rewarder_address) = self.get_rewarder(pool_id) {
            RewarderRef::on_arsw_reward(&rewarder_address, pool_id, caller, to, 0, 0)?;
        }

        let lp_token = self
            .get_lp_token(pool_id)
            .ok_or(FarmingError::LpTokenNotFound)?;
        PSP22Ref::transfer(&lp_token, to, amount, Vec::new())?;

        self._emit_emergency_withdraw_event(caller, pool_id, amount, to);
        Ok(())
    }

    #[ink(message)]
    #[modifiers(only_owner)]
    fn deposit_arsw(&mut self, amount: Balance) -> Result<(), FarmingError> {
        ensure!(amount > 0, FarmingError::ZeroWithdrawal);
        let caller = Self::env().caller();
        PSP22Ref::transfer_from_builder(
            &mut self.data::<Data>().arsw_token,
            caller,
            Self::env().account_id(),
            amount,
            Vec::new(),
        )
        .call_flags(CallFlags::default().set_allow_reentry(true))
        .fire()
        .unwrap()?;
        self._emit_deposit_arsw_event(Self::env().block_number(), amount);
        Ok(())
    }

    fn _check_pool_duplicate(&self, lp_token: AccountId) -> Result<(), FarmingError> {
        let lp_tokens = &self.data::<Data>().lp_tokens;
        ensure!(
            !lp_tokens.iter().any(|lp| *lp == lp_token),
            FarmingError::DuplicateLPToken
        );
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
        let mut pool = self
            .get_pool_info(pool_id)
            .ok_or(FarmingError::PoolNotFound)?;
        let current_block = Self::env().block_number();
        if current_block > pool.last_reward_block {
            let lp_supply = self._get_lp_supply(pool_id)?;
            if lp_supply > 0 {
                let additional_acc_arsw_per_share =
                    self._calculate_additional_acc_arsw_per_share(&pool, current_block, lp_supply)?;
                pool.acc_arsw_per_share = pool
                    .acc_arsw_per_share
                    .checked_add(additional_acc_arsw_per_share)
                    .ok_or(FarmingError::AddOverflow6)?;
            }
            pool.last_reward_block = current_block;
            self.data::<Data>().pool_info.insert(&pool_id, &pool);

            self._emit_log_update_pool_event(
                pool_id,
                pool.last_reward_block,
                lp_supply,
                pool.acc_arsw_per_share,
            );
        }
        Ok(())
    }

    fn _calculate_additional_acc_arsw_per_share(
        &self,
        pool_info: &Pool,
        current_block: u32,
        lp_supply: Balance,
    ) -> Result<Balance, FarmingError> {
        ensure!(lp_supply > 0, FarmingError::LpSupplyIsZero);
        let current_period = self._get_period(current_block)?;
        let mut arsw_reward: Balance = 0;
        let mut last_block = pool_info.last_reward_block;
        let last_reward_block_period = self._get_period(last_block)?;
        let mut period = last_reward_block_period;
        let total_alloc_point = self.get_total_alloc_point();
        while period <= current_period {
            if period > MAX_PERIOD {
                break
            }
            if current_block <= self._period_max(period)? {
                arsw_reward = arsw_reward
                    .checked_add(
                        casted_mul(
                            current_block
                                .checked_sub(last_block)
                                .ok_or(FarmingError::SubUnderflow2)?
                                as u128
                                * pool_info.alloc_point as u128,
                            self._arsw_per_block(period)?,
                        )
                        .checked_div(total_alloc_point.into())
                        .ok_or(FarmingError::DivByZero1)?
                        .try_into()
                        .map_err(|_| FarmingError::CastTou128Error1)?,
                    )
                    .ok_or(FarmingError::AddOverflow4)?;
            } else {
                arsw_reward = arsw_reward
                    .checked_add(
                        casted_mul(
                            self._period_max(period)?
                                .checked_sub(last_block.into())
                                .ok_or(FarmingError::SubUnderflow3)?
                                as u128
                                * pool_info.alloc_point as u128,
                            self._arsw_per_block(period)? as u128,
                        )
                        .checked_div(total_alloc_point.into())
                        .ok_or(FarmingError::DivByZero2)?
                        .try_into()
                        .map_err(|_| FarmingError::CastTou128Error2)?,
                    )
                    .ok_or(FarmingError::AddOverflow5)?;

                last_block = self._period_max(period)?;
            }

            period += 1;
        }
        Ok(
            (casted_mul(arsw_reward, ACC_ARSW_PRECISION) / U256::from(lp_supply))
                .try_into()
                .map_err(|_| FarmingError::CastTou128Error4)?,
        )
    }

    fn _get_period(&self, block_number: u32) -> Result<u32, FarmingError> {
        ensure!(
            block_number >= ARTHSWAP_ORIGIN_BLOCK,
            FarmingError::BlockNumberLowerThanOriginBlock
        );

        // BLOCK_PER_PERIOD is never 0
        return Ok(block_number
            .checked_sub(ARTHSWAP_ORIGIN_BLOCK)
            .ok_or(FarmingError::SubUnderflow1)?
            / BLOCK_PER_PERIOD)
    }

    fn _period_max(&self, period: u32) -> Result<u32, FarmingError> {
        Ok(ARTHSWAP_ORIGIN_BLOCK
            .checked_add(
                BLOCK_PER_PERIOD
                    .checked_mul(period.checked_add(1).ok_or(FarmingError::AddOverflow2)?)
                    .ok_or(FarmingError::MulOverflow1)?,
            )
            .ok_or(FarmingError::AddOverflow3)?
            - 1)
    }

    fn _arsw_per_block(&self, period: u32) -> Result<Balance, FarmingError> {
        if period > MAX_PERIOD {
            return Ok(0)
        }
        Ok((casted_mul(
            FIRST_PERIOD_REWERD_SUPPLY,
            9u128
                .checked_pow(period)
                .ok_or(FarmingError::PowOverflow1)?,
        ) / 10u128
            .checked_pow(period)
            .ok_or(FarmingError::PowOverflow2)?)
        .try_into()
        .map_err(|_| FarmingError::CastTou128Error3)?)
    }

    fn _get_lp_supply(&self, pool_id: u32) -> Result<Balance, FarmingError> {
        let lp_token = self
            .get_lp_token(pool_id)
            .ok_or(FarmingError::LpTokenNotFound)?;
        Ok(PSP22Ref::balance_of(&lp_token, Self::env().account_id()))
    }
}
