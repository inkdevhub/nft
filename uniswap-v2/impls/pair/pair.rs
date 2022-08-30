pub use crate::{
    impls::pair::*,
    traits::pair::*,
};
use openbrush::{
    contracts::{
        ownable::*,
        pausable::*,
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

impl<T: Storage<data::Data> + Storage<pausable::Data> + Storage<ownable::Data>> Pair for T {
    default fn get_reserves(&self) -> (Balance, Balance, Timestamp) {
        (
            self.data::<data::Data>().reserve_0,
            self.data::<data::Data>().reserve_1,
            self.data::<data::Data>().block_timestamp_last,
        )
    }

    #[modifiers(only_owner)]
    default fn initialize(&mut self, token_0: AccountId, token_1: AccountId) -> Result<(), PairError> {
        self.data::<data::Data>().token_0 = token_0;
        self.data::<data::Data>().token_1 = token_1;
        Ok(())
    }

    #[modifiers(when_not_paused)]
    default fn mint(&mut self, to: AccountId) -> Result<Balance, PairError> {
        let reserves = self.get_reserves();
        let contract = Self::env().account_id();
        let balance_0 = PSP22Ref::balance_of(&self.data::<data::Data>().token_0, contract);
        let balance_1 = PSP22Ref::balance_of(&self.data::<data::Data>().token_1, contract);

        let amount_0 = balance_0.checked_sub(reserves.0).ok_or(PairError::SubUnderFlow1)?;
        let amount_1 = balance_1.checked_sub(reserves.1).ok_or(PairError::SubUnderFlow2)?;
        Ok(amount_1)
    }

    default fn _mint_fee(&mut self, reserve_0: Balance, reserve_1: Balance) -> Result<bool, PairError> {
        // TODO update when factory contract is done  address feeTo = IUniswapV2Factory(factory).feeTo();
        Ok(true)
    }
}
