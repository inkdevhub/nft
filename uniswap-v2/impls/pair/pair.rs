pub use crate::{
    impls::pair::*,
    traits::pair::*,
};
use ink_prelude::vec::Vec;
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
// Zero Adress [0; 32] will prevent us from calling default fn of psps
// It is possible to override these functions but it is not needed it this case
pub const ZERO_ADDRESS: [u8; 32] = [1; 32];

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

        let liquidity;
        if total_supply == 0 {
            let liq = amount_0
                .checked_mul(amount_1)
                .ok_or(PairError::MulOverFlow1)?
                .checked_sub(MINIMUM_LIQUIDITY)
                .ok_or(PairError::SubUnderFlow3)?;
            liquidity = sqrt(liq);
            self._mint(ZERO_ADDRESS.into(), MINIMUM_LIQUIDITY)?;
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
            liquidity = min(liquidity_1, liquidity_2);
        }

        if liquidity == 0 {
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

    #[modifiers(when_not_paused)]
    default fn burn(
        &mut self,
        to: AccountId,
    ) -> Result<(Balance, Balance), PairError> {
        let reserves = self.get_reserves();
        let contract = Self::env().account_id();
        let token_0 = self.data::<data::Data>().token_0;
        let token_1 = self.data::<data::Data>().token_1;
        let mut balance_0 = PSP22Ref::balance_of(&token_0, contract);
        let mut balance_1 = PSP22Ref::balance_of(&token_1, contract);
        let liquidity = self._balance_of(&contract);

        let fee_on = self._mint_fee(reserves.0, reserves.1)?;
        let total_supply = self.data::<psp22::Data>().supply;
        let amount_0 = liquidity
            .checked_mul(balance_0)
            .ok_or(PairError::MulOverFlow6)?
            .checked_div(total_supply)
            .ok_or(PairError::DivByZero3)?;
        let amount_1 = liquidity
            .checked_mul(balance_1)
            .ok_or(PairError::MulOverFlow7)?
            .checked_div(total_supply)
            .ok_or(PairError::DivByZero4)?;

        if amount_0 == 0 || amount_0 == 0 {
            return Err(PairError::InsufficientLiquidityBurned)
        }

        self._burn_from(contract, liquidity)?;

        self._transfer_from_to(token_0, to, amount_0, Vec::<u8>::new())?;
        self._transfer_from_to(token_1, to, amount_1, Vec::<u8>::new())?;

        balance_0 = PSP22Ref::balance_of(&token_0, contract);
        balance_1 = PSP22Ref::balance_of(&token_1, contract);

        self._update(balance_0, balance_1, reserves.0, reserves.1)?;

        if fee_on {
            let k = reserves
                .0
                .checked_mul(reserves.1)
                .ok_or(PairError::MulOverFlow5)?;
            self.data::<data::Data>().k_last = k;
        }

        self._emit_burn_event(Self::env().caller(), amount_0, amount_1, to);

        Ok((amount_0, amount_1))
    }

    #[modifiers(when_not_paused)]
    fn swap(
        &mut self,
        amount_0_out: Balance,
        amount_1_out: Balance,
        to: AccountId,
    ) -> Result<(), PairError> {
        if amount_0_out == 0 && amount_1_out == 0 {
            return Err(PairError::InsufficientOutputAmount)
        }
        let reserves = self.get_reserves();
        if amount_0_out >= reserves.0 || amount_1_out >= reserves.1 {
            return Err(PairError::InsufficientLiquidity)
        }

        let token_0 = self.data::<data::Data>().token_0;
        let token_1 = self.data::<data::Data>().token_1;

        if to == token_0 || to == token_1 {
            return Err(PairError::InvalidTo)
        }
        if amount_0_out > 0 {
            self._transfer_from_to(
                token_0,
                to,
                amount_0_out,
                Vec::<u8>::new(),
            )?;
        }
        if amount_1_out > 0 {
            self._transfer_from_to(
                token_1,
                to,
                amount_1_out,
                Vec::<u8>::new(),
            )?;
        }
        let contract = Self::env().account_id();
        let balance_0 = PSP22Ref::balance_of(&token_0, contract);
        let balance_1 = PSP22Ref::balance_of(&token_1, contract);

        let amount_0_in = if balance_0
            > reserves
                .0
                .checked_sub(amount_0_out)
                .ok_or(PairError::SubUnderFlow4)?
        {
            balance_0
                .checked_sub(
                    reserves
                        .0
                        .checked_sub(amount_0_out)
                        .ok_or(PairError::SubUnderFlow5)?,
                )
                .ok_or(PairError::SubUnderFlow6)?
        } else {
            0
        };
        let amount_1_in = if balance_1
            > reserves
                .1
                .checked_sub(amount_1_out)
                .ok_or(PairError::SubUnderFlow7)?
        {
            balance_1
                .checked_sub(
                    reserves
                        .1
                        .checked_sub(amount_1_out)
                        .ok_or(PairError::SubUnderFlow8)?,
                )
                .ok_or(PairError::SubUnderFlow9)?
        } else {
            0
        };
        if amount_0_in == 0 && amount_1_in == 0 {
            return Err(PairError::InsufficientInputAmount)
        }

        let balance_0_adjusted = balance_0
            .checked_mul(1000)
            .ok_or(PairError::MulOverFlow8)?
            .checked_sub(
                amount_0_in.checked_mul(3).ok_or(PairError::MulOverFlow9)?,
            )
            .ok_or(PairError::SubUnderFlow10)?;
        let balance_1_adjusted = balance_1
            .checked_mul(1000)
            .ok_or(PairError::MulOverFlow10)?
            .checked_sub(
                amount_1_in.checked_mul(3).ok_or(PairError::MulOverFlow11)?,
            )
            .ok_or(PairError::SubUnderFlow11)?;

        if balance_0_adjusted
            .checked_mul(balance_1_adjusted)
            .ok_or(PairError::MulOverFlow12)?
            < reserves
                .0
                .checked_mul(reserves.1)
                .ok_or(PairError::MulOverFlow13)?
        {
            return Err(PairError::K)
        }

        self._update(balance_0, balance_1, reserves.0, reserves.1)?;

        self._emit_swap_event(
            Self::env().caller(),
            amount_0_in,
            amount_1_in,
            amount_0_out,
            amount_1_out,
            to,
        );
        Ok(())
    }

    default fn _mint_fee(
        &mut self,
        _reserve_0: Balance,
        _reserve_1: Balance,
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
