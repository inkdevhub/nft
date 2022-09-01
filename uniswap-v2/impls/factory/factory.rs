use crate::traits::pair::PairRef;
pub use crate::{
    impls::factory::*,
    traits::factory::*,
};
use openbrush::traits::{
    AccountId,
    Storage,
    ZERO_ADDRESS,
};

impl<T: Storage<data::Data>> Factory for T {
    fn all_pair_length(&self) -> u64 {
        self.data::<data::Data>().all_pairs.len() as u64
    }

    fn create_pair(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
    ) -> Result<AccountId, FactoryError> {
        if token_a == token_b {
            return Err(FactoryError::IdenticalAddresses)
        }
        let token_pair = if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        };
        if token_pair.0 == ZERO_ADDRESS.into() {
            return Err(FactoryError::ZeroAddress)
        }

        let random_seed = Self::env().random(Self::env().caller().as_ref());
        let mut pair_contract = self._instantiate_pair(random_seed.0.as_ref());

        PairRef::initialize(&mut pair_contract, token_pair.0, token_pair.1)?;

        self.data::<data::Data>()
            .get_pair
            .insert(&(token_pair.0, token_pair.1), &pair_contract);
        self.data::<data::Data>()
            .get_pair
            .insert(&(token_pair.1, token_pair.0), &pair_contract);
        self.data::<data::Data>().all_pairs.push(pair_contract);

        Ok(pair_contract)
    }

    default fn _instantiate_pair(&mut self, _salt_bytes: &[u8]) -> AccountId {
        // need to be overridden in contract
        unimplemented!()
    }
}
