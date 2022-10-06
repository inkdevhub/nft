use crate::{traits::factory::FactoryRef};
use crate::traits::pair::PairRef;
pub use crate::{
    impls::router::*,
    traits::router::*,
};
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::{
        traits::psp22::PSP22Ref,
    },
    traits::{
        AccountId,
        Balance,
        Storage,
        ZERO_ADDRESS,
    },
};
use primitive_types::U256;

impl<T: Storage<data::Data>> Router for T
{
    default fn factory(&self) -> AccountId {
        self.data::<data::Data>().factory
    }

    default fn quote(
        &self,
        amount_a: Balance,
        reserve_a: Balance,
        reserve_b: Balance
    ) -> Result<Balance, RouterError> {
        self._quote(amount_a, reserve_a, reserve_b)
    }

    default fn get_amount_out(
        &self,
        amount_in: Balance,
        reserve_in: Balance,
        reserve_out: Balance,
    ) -> Result<Balance, RouterError> {
        self._get_amount_out(amount_in, reserve_in, reserve_out)
    }

    default fn get_amount_in(
        &self,
        amount_out: Balance,
        reserve_in: Balance,
        reserve_out: Balance
    ) -> Result<Balance, RouterError> {
        self._get_amount_in(amount_out, reserve_in, reserve_out)
    }

    default fn add_liquidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        amount_a_desired: Balance,
        amount_b_desired: Balance,
        amount_a_min: Balance,
        amount_b_min: Balance,
        to: AccountId,
        dead_line: u64,
    ) ->  Result<(Balance, Balance, Balance), RouterError> {
        if dead_line <= Self::env().block_timestamp() {
            return Err(RouterError::Expired)
        }

        let (amount_a, amount_b) = self._add_liquidity(
            token_a,
            token_b,
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
        )?;

        let factory = self.data::<data::Data>().factory;
        let pair_contract = self._pair_for(factory, token_a, token_b)?;

        PSP22Ref::transfer_from(
            &token_a,
            Self::env().caller(),
            pair_contract,
            amount_a,
            Vec::new(),
        )?;
        PSP22Ref::transfer_from(
            &token_b,
            Self::env().caller(),
            pair_contract,
            amount_b,
            Vec::new(),
        )?;

        let liquidity = PairRef::mint(&pair_contract, to)?;
        
        ink_env::debug_println!("Liquidity {:?}", liquidity);

        Ok((amount_a, amount_b, liquidity))
    }

    default fn remove_lequidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        liquidity: Balance,
        amount_a_min: Balance,
        amount_b_min: Balance,
        to: AccountId,
        dead_line: u64,
    ) -> Result<(Balance, Balance), RouterError> {
        if dead_line <= Self::env().block_timestamp() {
            return Err(RouterError::Expired)
        }

        let factory = self.data::<data::Data>().factory;
        let pair_contract = self._pair_for(factory, token_a, token_b)?;

        PSP22Ref::transfer_from(
            &pair_contract,
            Self::env().caller(),
            pair_contract,
            liquidity,
            Vec::new(),
        )?;

        let (amount_0, amount_1) = PairRef::burn(&pair_contract, to)?;
        let (token_0, _) = self._sort_tokens(token_a, token_b)?;
        let (amount_a, amount_b) = if token_a == token_0 {
            (amount_0, amount_1)
        } else {
            (amount_1, amount_0)
        };

        if amount_a < amount_a_min {
            return Err(RouterError::InsufficientAAmount)
        }
        if amount_b < amount_b_min {
            return Err(RouterError::InsufficientBAmount)
        }

        Ok((amount_a, amount_b))
    }

    default fn _quote(
        &self,
        amount_a: Balance,
        reserve_a: Balance,
        reserve_b: Balance,
    ) -> Result<Balance, RouterError> {
        if amount_a <= 0 {
            return Err(RouterError::InsufficientAmount)
        }
        if reserve_a <= 0 || reserve_b <= 0 {
            return Err(RouterError::InsufficientLiquidity)
        }

        let amount_b: Balance = casted_mul(amount_a, reserve_b)
            .checked_div(U256::from(reserve_a))
            .ok_or(RouterError::DivByZero)?
            .as_u128();

        Ok(amount_b)
    }

    default fn _get_amount_out(
        &self,
        amount_in: Balance,
        reserve_in: Balance,
        reserve_out: Balance,
    ) -> Result<Balance, RouterError> {
        if amount_in <= 0 {
            return Err(RouterError::InsufficientAmount)
        }
        if reserve_in <= 0 || reserve_out <= 0 {
            return Err(RouterError::InsufficientLiquidity)
        }

        let amount_in_with_fee = casted_mul(amount_in, 997);

        let numerator = amount_in_with_fee
            .checked_mul(U256::from(reserve_out))
            .ok_or(RouterError::MulOverFlow)?;

        let denominator = casted_mul(reserve_in, 1000)
            .checked_add(amount_in_with_fee)
            .ok_or(RouterError::AddOverFlow)?;

        let amount_out: Balance = numerator
            .checked_div(denominator)
            .ok_or(RouterError::DivByZero)?
            .as_u128();

        Ok(amount_out)
    }

    default fn _get_amount_in(
        &self,
        amount_out: Balance,
        reserve_in: Balance,
        reserve_out: Balance
    ) -> Result<Balance, RouterError> {
        if amount_out <= 0 {
            return Err(RouterError::InsufficientAmount)
        }
        if reserve_in <= 0 || reserve_out <= 0 {
            return Err(RouterError::InsufficientLiquidity)
        }

        let numerator: U256 = U256::from(reserve_in)
            .checked_mul(U256::from(amount_out))
            .ok_or(RouterError::MulOverFlow)?
            .checked_mul(U256::from(1000))
            .ok_or(RouterError::MulOverFlow)?;

        let denominator: U256 = U256::from(reserve_out)
            .checked_sub(U256::from(amount_out))
            .ok_or(RouterError::SubUnderFlow)?
            .checked_mul(U256::from(997))
            .ok_or(RouterError::MulOverFlow)?;

        let amount_in: Balance = numerator
            .checked_div(denominator)
            .ok_or(RouterError::DivByZero)?
            .checked_add(U256::from(1 as Balance))
            .ok_or(RouterError::AddOverFlow)?
            .as_u128();

        Ok(amount_in)    
    }

    default fn _add_liquidity(
        &self,
        token_a: AccountId,
        token_b: AccountId,
        amount_a_desired: Balance,
        amount_b_desired: Balance,
        amount_a_min: Balance,
        amount_b_min: Balance,
    ) -> Result<(Balance, Balance), RouterError> {
        if FactoryRef::get_pair(&self.data::<data::Data>().factory, token_a, token_b).is_none() {
            FactoryRef::create_pair(&self.data::<data::Data>().factory, token_a, token_b)?;
        };

        let (reserve_a, reserve_b) = self._get_reserves(self.data::<data::Data>().factory, token_a, token_b)?;
        if reserve_a == 0 && reserve_b == 0 {
            return Ok((amount_a_desired, amount_b_desired))
        }

        let amount_b_optimal = self._quote(amount_a_desired, reserve_a, reserve_b)?;
        if amount_b_optimal <= amount_b_desired {
            if amount_b_optimal < amount_b_min {
                return Err(RouterError::InsufficientBAmount)
            }

            Ok((amount_a_desired, amount_b_optimal))
        } else {
            let amount_a_optimal = self._quote(amount_b_desired, reserve_b, reserve_a)?;
            assert!(amount_a_optimal <= amount_a_desired);
            if amount_a_optimal < amount_a_min {
                return Err(RouterError::InsufficientAAmount)
            }

            Ok((amount_a_optimal, amount_b_desired))
        }
    }

    default fn _pair_for(
        &self,
        factory: AccountId,
        token_a: AccountId,
        token_b: AccountId,
    ) -> Result<AccountId, RouterError> {
        let (token_0, token_1) = self._sort_tokens(token_a, token_b)?;

        // Original Uniswap Library pairFor function calculate pair contract address without making external calls.
        // Please refer https://github.com/Uniswap/v2-periphery/blob/master/contracts/libraries/UniswapV2Library.sol#L18

        // In this contract, use external call to get pair contract address.
        let pair = FactoryRef::get_pair(&factory, token_0, token_1)
            .ok_or(RouterError::PairNotFound)?;
        Ok(pair)
    }

    default fn _sort_tokens(
        &self,
        token_a: AccountId,
        token_b: AccountId,
    ) -> Result<(AccountId, AccountId), RouterError> {
        if token_a == token_b {
            return Err(RouterError::IdenticalAddresses)
        }

        let (token_0, token_1) = if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        };

        if token_0 == ZERO_ADDRESS.into() {
            return Err(RouterError::ZeroAddress)
        }

        Ok((token_0, token_1))
    }

    default fn _get_reserves(
        &self,
        factory: AccountId,
        token_a: AccountId,
        token_b: AccountId,
    ) -> Result<(Balance, Balance), RouterError> {
        let (token_0, _) = self._sort_tokens(token_a, token_b)?;
        // get pair contract address from factory
        let pair_contract = self._pair_for(factory, token_a, token_b)?;
        let (reserve_0, reserve_1, _) = PairRef::get_reserves(&pair_contract);

        if token_a == token_0 {
            Ok((reserve_0, reserve_1))
        } else {
            Ok((reserve_1, reserve_0))
        }
    }
}

fn casted_mul(a: u128, b: u128) -> U256 {
    U256::from(a) * U256::from(b)
}