pub use crate::{
    impls::pair::*,
    traits::pair::*,
};
use openbrush::{
    contracts::{
        ownable::*,
        pausable::*,
        psp22::*,
        traits::psp22::PSP22Ref,
    },
    modifiers,
    traits::{
        AccountId,
        Balance,
        Storage,
        Timestamp,
    },
};

pub const MINIMUM_LIQUIDITY: u128 = 1000;

impl<
        T: Storage<data::Data>
            + Storage<pausable::Data>
            + Storage<ownable::Data>
            + Storage<psp22::Data>,
    > Pair for T
{
    default fn get_reserves(&self) -> (Balance, Balance, Timestamp) {
        (
            self.data::<data::Data>().reserve_0,
            self.data::<data::Data>().reserve_1,
            self.data::<data::Data>().block_timestamp_last,
        )
    }

    #[modifiers(only_owner)]
    default fn initialize(
        &mut self,
        token_0: AccountId,
        token_1: AccountId,
    ) -> Result<(), PairError> {
        self.data::<data::Data>().token_0 = token_0;
        self.data::<data::Data>().token_1 = token_1;
        Ok(())
    }

    #[modifiers(when_not_paused)]
    default fn mint(&mut self, to: AccountId) -> Result<Balance, PairError> {
        let reserves = self.get_reserves();
        let contract = Self::env().account_id();
        let balance_0 =
            PSP22Ref::balance_of(&self.data::<data::Data>().token_0, contract);
        let balance_1 =
            PSP22Ref::balance_of(&self.data::<data::Data>().token_1, contract);
        let amount_0 = balance_0
            .checked_sub(reserves.0)
            .ok_or(PairError::SubUnderFlow1)?;
        let amount_1 = balance_1
            .checked_sub(reserves.1)
            .ok_or(PairError::SubUnderFlow2)?;

        let fee_on = self._mint_fee(reserves.0, reserves.1)?;
        let total_supply = self.data::<psp22::Data>().supply;

        let mut liquidity;
        if total_supply == 0 {
            let liq = amount_0
                .checked_mul(amount_1)
                .ok_or(PairError::MulOverFlow1)?
                .checked_sub(MINIMUM_LIQUIDITY)
                .ok_or(PairError::SubUnderFlow3)?;
            liquidity = sqrt(liq);
        } else {
            let liquidity_1 = amount_0
                .checked_mul(total_supply)
                .ok_or(PairError::MulOverFlow2)?
                .checked_div(reserves.0)
                .ok_or(PairError::DivByZero1)?;
            let liquidity_2 = amount_1
                .checked_mul(total_supply)
                .ok_or(PairError::MulOverFlow3)?
                .checked_div(reserves.1)
                .ok_or(PairError::DivByZero2)?;
            liquidity = min(amount_0, amount_1);
        }

        if liquidity <= 0 {
            return Err(PairError::InsufficientLiquidityMinted)
        }

        self._mint(to, liquidity)?;

        self._update(balance_0, balance_1, reserves.0, reserves.1)?;

        if fee_on {
            let k = reserves
                .0
                .checked_mul(reserves.1)
                .ok_or(PairError::MulOverFlow5)?;
            self.data::<data::Data>().k_last = k;
        }

        self._emit_mint_event(Self::env().caller(), amount_0, amount_1);

        Ok(liquidity)
    }

    default fn _mint_fee(
        &mut self,
        reserve_0: Balance,
        reserve_1: Balance,
    ) -> Result<bool, PairError> {
        // TODO update when factory contract is done
        Ok(true)
    }

    default fn _update(
        &mut self,
        balance_0: Balance,
        balance_1: Balance,
        reserve_0: Balance,
        reserve_1: Balance,
    ) -> Result<(), PairError> {
        if balance_0 == u128::MAX || balance_1 == u128::MAX {
            return Err(PairError::Overflow)
        }
        let now = Self::env().block_timestamp();
        let time_elapsed = now - self.data::<data::Data>().block_timestamp_last;
        if time_elapsed > 0 && reserve_0 != 0 && reserve_1 != 0 {
            let price_cumulative_last_0 = (reserve_1 / reserve_0)
                .checked_mul(time_elapsed as u128)
                .ok_or(PairError::MulOverFlow4)?;
            let price_cumulative_last_1 = (reserve_0 / reserve_1)
                .checked_mul(time_elapsed as u128)
                .ok_or(PairError::MulOverFlow4)?;
            self.data::<data::Data>().price_0_cumulative_last =
                price_cumulative_last_0;
            self.data::<data::Data>().price_1_cumulative_last =
                price_cumulative_last_1;
        }
        self.data::<data::Data>().reserve_0 = balance_0;
        self.data::<data::Data>().reserve_1 = balance_1;
        self.data::<data::Data>().block_timestamp_last = now;

        Ok(())
    }

    default fn _emit_mint_event(
        &self,
        _sender: AccountId,
        _amount_0: Balance,
        _amount_1: Balance,
    ) {
    }
    default fn _emit_burn_event(
        &self,
        _sender: AccountId,
        _amount_0: Balance,
        _amount_1: Balance,
        _to: AccountId,
    ) {
    }
    default fn _emit_swap_event(
        &self,
        _sender: AccountId,
        _amount_0_in: Balance,
        _amount_1_in: Balance,
        _amount_0_out: Balance,
        _amount_1_out: Balance,
        _to: AccountId,
    ) {
    }
}

fn min(x: u128, y: u128) -> u128 {
    if x < y {
        return x
    }
    y
}

fn sqrt(y: u128) -> u128 {
    let mut z = 1;
    if y > 3 {
        z = y;
        let mut x = y / 2 + 1;
        while x < z {
            z = x;
            x = (y / x + x) / 2;
        }
    }
    z
}
