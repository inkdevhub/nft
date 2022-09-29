#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod router {
    use primitive_types::U256;
    use ink_env::CallFlags;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            psp22::PSP22Error,
            traits::psp22::PSP22Ref,
        },
        traits::ZERO_ADDRESS,
    };

    use uniswap_v2::traits::{
        factory::{
            FactoryError,
            FactoryRef,
        },
        pair::{
            PairError,
            PairRef,
        },
    };

    // Error Definition
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum RouterError {
        PSP22Error(PSP22Error),
        FactoryError(FactoryError),
        PairError(PairError),
        PairNotFound,
        InsufficientAmount,
        InsufficientAAmount,
        InsufficientBAmount,
        InsufficientLiquidity,
        ZeroAddress,
        IdenticalAddresses,
        Expired,
        AddOverFlow,
        SubUnderFlow,
        MulOverFlow,
        DivByZero,
    }

    impl From<PSP22Error> for RouterError {
        fn from(error: PSP22Error) -> Self {
            RouterError::PSP22Error(error)
        }
    }

    impl From<FactoryError> for RouterError {
        fn from(error: FactoryError) -> Self {
            RouterError::FactoryError(error)
        }
    }

    impl From<PairError> for RouterError {
        fn from(error: PairError) -> Self {
            RouterError::PairError(error)
        }
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct Router {
        factory: AccountId,
    }

    impl Router {
        #[ink(constructor)]
        pub fn new(factory: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.factory = factory;
            })
        }

        #[ink(message)]
        pub fn factory(&self) -> AccountId {
            self.factory
        }

        #[ink(message)]
        pub fn quote(
            &self,
            amount_a: Balance,
            reserve_a: Balance,
            reserve_b: Balance,
        ) -> Result<Balance, RouterError> {
            self._quote(amount_a, reserve_a, reserve_b)
        }

        #[ink(message)]
        pub fn get_amount_out(
            &self,
            amount_in: Balance,
            reserve_in: Balance,
            reserve_out: Balance,
        ) -> Result<Balance, RouterError> {
            self._get_amount_out(amount_in, reserve_in, reserve_out)
        }

        #[ink(message)]
        pub fn get_amount_in(
            &self,
            amount_out: Balance,
            reserve_in: Balance,
            reserve_out: Balance,
        ) -> Result<Balance, RouterError> {
            self._get_amount_in(amount_out, reserve_in, reserve_out)
        }

        #[ink(message)]
        pub fn add_liquidity(
            &mut self,
            token_a: AccountId,
            token_b: AccountId,
            amount_a_desired: Balance,
            amount_b_desired: Balance,
            amount_a_min: Balance,
            amount_b_min: Balance,
        ) -> Result<(Balance, Balance, Balance), RouterError> {
            let (amount_a, amount_b) = self._add_liquidity(
                token_a,
                token_b,
                amount_a_desired,
                amount_b_desired,
                amount_a_min,
                amount_b_min,
            )?;

            let pair_contract = self._pair_for(self.factory, token_a, token_b)?;

            // self._safe_transfer(token_a, pair_contract, amount_a)?;
            // self._safe_transfer(token_b, pair_contract, amount_b)?;
            PSP22Ref::transfer_from(
                &token_a,
                self.env().caller(),
                pair_contract,
                amount_a,
                Vec::new(),
            )?;
            PSP22Ref::transfer_from(
                &token_b,
                self.env().caller(),
                pair_contract,
                amount_b,
                Vec::new(),
            )?;

            let liquidity = PairRef::mint(&pair_contract, self.env().caller())?;

            Ok((amount_a, amount_b, liquidity))
        }

        #[ink(message)]
        pub fn remove_requidity(
            &mut self,
            token_a: AccountId,
            token_b: AccountId,
            liquidity: Balance,
            amount_a_min: Balance,
            amount_b_min: Balance,
            to: AccountId,
            dead_line: u64,
        ) -> Result<(Balance, Balance), RouterError> {
            if dead_line <= self.env().block_timestamp() {
                return Err(RouterError::Expired)
            }

            let pair_contract = self._pair_for(self.factory, token_a, token_b)?;
            PSP22Ref::transfer_from(
                &pair_contract,
                self.env().caller(),
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

        fn _quote(
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

            ink_env::debug_println!("Balance amount_a {:?}", amount_a);
            ink_env::debug_println!("Balance reserve_a {:?}", reserve_a);
            ink_env::debug_println!("Balance reserve_b {:?}", reserve_b);

            let amount_b: Balance = U256::from(amount_a)
                .checked_mul(U256::from(reserve_b))
                .ok_or(RouterError::MulOverFlow)?
                .checked_div(U256::from(reserve_a))
                .ok_or(RouterError::DivByZero)?
                .as_u128();

            ink_env::debug_println!("Result: Balance amount_b {:?}", amount_b);

            Ok(amount_b)
        }
        
        fn _get_amount_out(
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

            let amount_in_with_fee = U256::from(amount_in)
                .checked_mul(U256::from(997))
                .ok_or(RouterError::MulOverFlow)?;

            let numerator = amount_in_with_fee
                .checked_mul(U256::from(reserve_out))
                .ok_or(RouterError::MulOverFlow)?;

            let denominator = U256::from(reserve_in)
                .checked_mul(U256::from(1000))
                .ok_or(RouterError::MulOverFlow)?
                + amount_in_with_fee;

            let amount_out: Balance = numerator
                .checked_div(denominator)
                .ok_or(RouterError::DivByZero)?
                .as_u128();

            Ok(amount_out)
        }

        fn _get_amount_in(
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
                .checked_add(U256::from(1))
                .ok_or(RouterError::AddOverFlow)?
                .as_u128();

            Ok(amount_in)
        }

        fn _get_reserves(
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

        fn _sort_tokens(
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

        fn _add_liquidity(
            &mut self,
            token_a: AccountId,
            token_b: AccountId,
            amount_a_desired: Balance,
            amount_b_desired: Balance,
            amount_a_min: Balance,
            amount_b_min: Balance,
        ) -> Result<(Balance, Balance), RouterError> {
            if FactoryRef::get_pair(&self.factory, token_a, token_b).is_none() {
                FactoryRef::create_pair(&self.factory, token_a, token_b)?;
            };

            let (reserve_a, reserve_b) = self._get_reserves(self.factory, token_a, token_b)?;
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

        fn _pair_for(
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

        fn _safe_transfer(
            &mut self,
            token: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<(), RouterError> {
            PSP22Ref::transfer_builder(&token, to, value, Vec::<u8>::new())
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .fire()
                .unwrap()?;
            Ok(())
        }
    }
}
