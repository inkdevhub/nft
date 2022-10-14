use crate::traits::farming::Data;
use openbrush::traits::Storage;

#[openbrush::trait_definition]
pub trait FarmingGetters: Storage<Data> {
    #[ink(message)]
    fn pool_length(&self) -> u32 {
        self.data::<Data>().pool_info_length
    }
}
