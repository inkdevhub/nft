use openbrush::traits::{AccountId, Balance, Storage, Timestamp};
use crate::impls::pair::data;
use crate::traits::pair::{Pair, PairError};

impl<T: Storage<data::Data>> Pair for T {
    default fn get_reserves(&self) ->(Balance, Balance, Timestamp) {
        (self.data().reserve_0, self.data().reserve_0, self.data().block_timestamp_last)
    }

    default fn initialize(&mut self, token_0: AccountId, token_1: AccountId) {
        self.data().token_0 = token_0;
        self.data().token_1 = token_1;
    }
}