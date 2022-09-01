pub use crate::{
    impls::factory::*,
    traits::factory::*,
};
use openbrush::traits::Storage;

impl<T: Storage<data::Data>> Factory for T {
    fn all_pair_length(&self) -> u128 {
        todo!()
    }
}
