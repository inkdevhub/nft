use crate::traits::{
    factory::FactoryRef,
    pair::PairRef,
};
pub use crate::{
    impls::router::*,
    traits::{
        math::*,
        router::*,
    },
};
use ink_env::hash::Blake2x256;
use ink_prelude::{
    vec,
    vec::Vec,
};
use openbrush::{
    contracts::traits::psp22::PSP22Ref,
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        Balance,
        Storage,
        ZERO_ADDRESS,
    },
};
use primitive_types::U256;

// Chain decimals is 18
pub const ONE: u128 = 1_000_000_000_000_000_000;

impl<T: Storage<data::Data>> Router for T {
    default fn factory(&self) -> AccountId {
        self.data::<data::Data>().factory
    }

    default fn quote(
        &self,
        amount_a: Balance,
        reserve_a: Balance,
        reserve_b: Balance,
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
        reserve_out: Balance,
    ) -> Result<Balance, RouterError> {
        self._get_amount_in(amount_out, reserve_in, reserve_out)
    }

    default fn get_amounts_out(
        &self,
        amount_in: Balance,
        path: Vec<AccountId>,
    ) -> Result<Vec<Balance>, RouterError> {
        let factory = self.data::<data::Data>().factory;
        self._get_amounts_out(factory, amount_in, &path)
    }

    default fn get_amounts_in(
        &self,
        amount_out: Balance,
        path: Vec<AccountId>,
    ) -> Result<Vec<Balance>, RouterError> {
        let factory = self.data::<data::Data>().factory;
        self._get_amounts_in(factory, amount_out, &path)
    }

    #[modifiers(ensure(deadline))]
    default fn add_liquidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        amount_a_desired: Balance,
        amount_b_desired: Balance,
        amount_a_min: Balance,
        amount_b_min: Balance,
        to: AccountId,
        deadline: u64,
    ) -> Result<(Balance, Balance, Balance), RouterError> {
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

        Ok((amount_a, amount_b, liquidity))
    }

    #[modifiers(ensure(deadline))]
    default fn remove_liquidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        liquidity: Balance,
        amount_a_min: Balance,
        amount_b_min: Balance,
        to: AccountId,
        deadline: u64,
    ) -> Result<(Balance, Balance), RouterError> {
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

    #[modifiers(ensure(deadline))]
    default fn swap_exact_tokens_for_tokens(
        &mut self,
        amount_in: Balance,
        amount_out_min: Balance,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u64,
    ) -> Result<Vec<Balance>, RouterError> {
        let factory = self.data::<data::Data>().factory;
        let amounts: Vec<Balance> = self._get_amounts_out(factory, amount_in, &path)?;
        if amounts[amounts.len() - 1] < amount_out_min {
            return Err(RouterError::InsufficientOutputAmount)
        }

        let pair_contract = self._pair_for(factory, path[0], path[1])?;
        PSP22Ref::transfer_from(
            &path[0],
            Self::env().caller(),
            pair_contract,
            amounts[0],
            Vec::new(),
        )?;

        self._swap(amounts.clone(), path, to)?;

        Ok(amounts)
    }

    #[modifiers(ensure(deadline))]
    default fn swap_tokens_for_exact_tokens(
        &mut self,
        amount_out: Balance,
        amount_in_max: Balance,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u64,
    ) -> Result<Vec<Balance>, RouterError> {
        let factory = self.data::<data::Data>().factory;
        let amounts: Vec<Balance> = self._get_amounts_out(factory, amount_out, &path)?;
        if amount_in_max < amounts[0] {
            return Err(RouterError::ExcessiveInputAmount)
        }

        let pair_contract = self._pair_for(factory, path[0], path[1])?;
        PSP22Ref::transfer_from(
            &path[0],
            Self::env().caller(),
            pair_contract,
            amounts[0],
            Vec::new(),
        )?;

        self._swap(amounts.clone(), path, to)?;

        Ok(amounts)
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
            .ok_or(RouterError::DivByZero1)?
            .try_into()
            .map_err(|_| RouterError::CastOverflow1)?;

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
            .ok_or(RouterError::MulOverFlow1)?;

        let denominator = casted_mul(reserve_in, 1000)
            .checked_add(amount_in_with_fee)
            .ok_or(RouterError::AddOverFlow1)?;

        let amount_out: Balance = numerator
            .checked_div(denominator)
            .ok_or(RouterError::DivByZero2)?
            .try_into()
            .map_err(|_| RouterError::CastOverflow2)?;

        Ok(amount_out)
    }

    default fn _get_amounts_out(
        &self,
        factory: AccountId,
        amount_in: Balance,
        path: &Vec<AccountId>,
    ) -> Result<Vec<Balance>, RouterError> {
        if path.len() < 2 {
            return Err(RouterError::InvalidPath)
        }

        let mut amounts: Vec<Balance> = vec![amount_in];
        for i in 0..path.len() - 1 {
            let (reserve_in, reserve_out) = self._get_reserves(factory, path[i], path[i + 1])?;
            amounts.push(self.get_amount_out(amounts[i], reserve_in, reserve_out)?);
        }

        Ok(amounts)
    }

    default fn _get_amounts_in(
        &self,
        factory: AccountId,
        amount_out: Balance,
        path: &Vec<AccountId>,
    ) -> Result<Vec<Balance>, RouterError> {
        if path.len() < 2 {
            return Err(RouterError::InvalidPath)
        }

        let mut amounts: Vec<Balance> = vec![amount_out];
        for i in 0..path.len() - 1 {
            let (reserve_in, reserve_out) = self._get_reserves(factory, path[i], path[i + 1])?;
            amounts.push(self.get_amount_in(amounts[i], reserve_in, reserve_out)?);
        }

        Ok(amounts)
    }

    default fn _get_amount_in(
        &self,
        amount_out: Balance,
        reserve_in: Balance,
        reserve_out: Balance,
    ) -> Result<Balance, RouterError> {
        if amount_out <= 0 {
            return Err(RouterError::InsufficientAmount)
        }
        if reserve_in <= 0 || reserve_out <= 0 {
            return Err(RouterError::InsufficientLiquidity)
        }

        let numerator: U256 = U256::from(reserve_in)
            .checked_mul(U256::from(amount_out))
            .ok_or(RouterError::MulOverFlow2)?
            .checked_mul(U256::from(1000))
            .ok_or(RouterError::MulOverFlow3)?;

        let denominator: U256 = U256::from(reserve_out)
            .checked_sub(U256::from(amount_out))
            .ok_or(RouterError::SubUnderFlow1)?
            .checked_mul(U256::from(997))
            .ok_or(RouterError::MulOverFlow4)?;

        let amount_in: Balance = numerator
            .checked_div(denominator)
            .ok_or(RouterError::DivByZero3)?
            .checked_add(U256::from(ONE as Balance))
            .ok_or(RouterError::AddOverFlow2)?
            .try_into()
            .map_err(|_| RouterError::CastOverflow3)?;

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
        let factory = self.data::<data::Data>().factory;
        if FactoryRef::get_pair(&factory, token_a, token_b).is_none() {
            FactoryRef::create_pair(&factory, token_a, token_b)?;
        };

        let (reserve_a, reserve_b) =
            self._get_reserves(self.data::<data::Data>().factory, token_a, token_b)?;
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
            if amount_a_desired < amount_a_optimal {
                return Err(RouterError::ExcessiveAAmount)
            }
            if amount_a_optimal < amount_a_min {
                return Err(RouterError::InsufficientAAmount)
            }

            Ok((amount_a_optimal, amount_b_desired))
        }
    }

    default fn _swap(
        &mut self,
        amounts: Vec<Balance>,
        path: Vec<AccountId>,
        to: AccountId,
    ) -> Result<(), RouterError> {
        let factory = self.data::<data::Data>().factory;

        for i in 0..path.len() - 1 {
            let (input, output) = (path[i], path[i + 1]);
            let (token_0, _) = self._sort_tokens(input, output)?;
            let amount_out: Balance = amounts[i + 1];

            let (amount_0_out, amount_1_out) = if input == token_0 {
                (0, amount_out)
            } else {
                (amount_out, 0)
            };

            let to = if i < path.len() - 2 {
                self._pair_for(factory, output, path[i + 2])?
            } else {
                to
            };

            PairRef::swap(
                &self._pair_for(factory, input, output)?,
                amount_0_out,
                amount_1_out,
                to,
            )?;
        }

        Ok(())
    }

    default fn _pair_for(
        &self,
        factory: AccountId,
        token_a: AccountId,
        token_b: AccountId,
    ) -> Result<AccountId, RouterError> {
        // TODO: remove pair code hash after deployment
        let tokens = self._sort_tokens(token_a, token_b)?;
        let salt = &Self::env().hash_encoded::<Blake2x256, _>(&tokens)[..4];
        let input: Vec<_> = AsRef::<[u8]>::as_ref(&factory)
            .iter()
            .chain(self.data().pair_code_hash.as_ref())
            .chain(salt)
            .cloned()
            .collect();
        Ok(Self::env().hash_bytes::<Blake2x256>(&input).into())
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
        let pair_contract = self._pair_for(factory, token_a, token_b)?;
        let (reserve_0, reserve_1, _) = PairRef::get_reserves(&pair_contract);

        if token_a == token_0 {
            Ok((reserve_0, reserve_1))
        } else {
            Ok((reserve_1, reserve_0))
        }
    }
}

#[modifier_definition]
pub fn ensure<T, F, R, E>(instance: &mut T, body: F, deadline: u64) -> Result<R, E>
where
    T: Storage<data::Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<RouterError>,
{
    if deadline <= T::env().block_timestamp() {
        return Err(From::from(RouterError::Expired))
    }
    body(instance)
}
