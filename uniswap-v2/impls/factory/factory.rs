use crate::traits::pair::PairRef;
pub use crate::{
    impls::factory::*,
    traits::factory::*,
};
use openbrush::{
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        Storage,
        ZERO_ADDRESS,
    },
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

    #[modifiers(only_fee_setter)]
    default fn set_fee_to(&mut self, fee_to: AccountId) -> Result<(), FactoryError> {
        self.data::<data::Data>().fee_to = fee_to;
        Ok(())
    }

    #[modifiers(only_fee_setter)]
    default fn set_fee_to_setter(&mut self, fee_to_setter: AccountId) -> Result<(), FactoryError> {
        self.data::<data::Data>().fee_to_setter = fee_to_setter;
        Ok(())
    }

    default fn fee_to(&self) -> AccountId {
        self.data::<data::Data>().fee_to
    }

    default fn fee_to_setter(&self) -> AccountId {
        self.data::<data::Data>().fee_to_setter
    }
}

#[modifier_definition]
pub fn only_fee_setter<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<data::Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<FactoryError>,
{
    if instance.data().fee_to_setter != T::env().caller() {
        return Err(From::from(FactoryError::CallerIsNotFeeSetter))
    }
    body(instance)
}
