#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod router {
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::vec::Vec;
    use openbrush::traits::{
        ZERO_ADDRESS
    };

    use uniswap_v2::traits::pair::PairRef;

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
        pub fn quote(&self, amount_a: Balance, reserve_a: Balance, reserve_b: Balance) -> Balance {
            self._quote(amount_a, reserve_a, reserve_b)
        }

        #[ink(message)]
        pub fn get_amount_out(&self, amount_in: Balance, reserve_in: Balance, reserve_out: Balance) -> Balance {
            self._get_amount_out(amount_in, reserve_in, reserve_out)
        }

        #[ink(message)]
        pub fn get_amount_in(&self, amount_out: Balance, reserve_in: Balance, reserve_out: Balance) -> Balance {
            self._get_amount_in(amount_out, reserve_in, reserve_out)
        }

        // TODO
        #[ink(message)]
        pub fn get_amounts_out(&self, factory: AccountId, amount_in: Balance, path: Vec<AccountId>) -> Vec<Balance> {
            Vec::<Balance>::new()
        }

        // TODO
        #[ink(message)]
        pub fn get_amounts_in(&self, factory: AccountId, amount_out: Balance, path: Vec<AccountId>) -> Vec<Balance> {
            Vec::<Balance>::new()
        }

        #[ink(message)]
        pub fn add_liquidity(&mut self, token_a: AccountId, token_b: AccountId, amount_a_desired: Balance, amount_b_desired: Balance, amount_a_min: Balance, amount_b_min: Balance) -> (Balance, Balance, Balance) {
            
        }

        fn _quote(&self, amount_a: Balance, reserve_a: Balance, reserve_b: Balance) -> Balance {
            assert!(0 < amount_a, "INSUFFICIENT AMOUNT");
            assert!(0 < reserve_a && 0 < reserve_b, "INSUFFICIENT LIQUIDITY");

            let amount_b = amount_a * reserve_b / reserve_a;
            amount_b
        }

        fn _get_amount_out(&self, amount_in: Balance, reserve_in: Balance, reserve_out: Balance) -> Balance {
            assert!(0 < amount_in, "INSUFFICIENT INPUT AMOUNT");
            assert!(0 < reserve_in && 0 < reserve_out, "INSUFFICIENT LIQUIDITY");

            let amount_in_with_fee = amount_in * 997;
            let numerator = amount_in_with_fee * reserve_out;
            let denominator = reserve_in * 1000 + amount_in_with_fee;

            let amount_out = numerator / denominator;
            amount_out
        }

        fn _get_amount_in(&self, amount_out: Balance, reserve_in: Balance, reserve_out: Balance) -> Balance {
            assert!(0 < amount_out, "INSUFFICIENT INPUT AMOUNT");
            assert!(0 < reserve_in && 0 < reserve_out, "INSUFFICIENT LIQUIDITY");

            let numerator = reserve_in * amount_out * 1000;
            let denominator = reserve_out - amount_out * 997;

            let amount_in = numerator / denominator + 1;
            amount_in
        }

        fn _get_reserves(&self, factory: AccountId, token_a: AccountId, token_b: AccountId) -> (Balance, Balance) {
            let (token_0, _) = self._sort_tokens(token_a, token_b);
            let (reserve_0, reserve_1, _) = PairRef::get_reserves(&factory);
            if token_a == token_0 { (reserve_0, reserve_1) } else { (reserve_1, reserve_0) }
        }

        fn _sort_tokens(&self, token_a: AccountId, token_b: AccountId) -> (AccountId, AccountId) {
            assert!(token_a != token_b, "IDENTICAL ADDRESSES");
            let (token_0, token_1) = if token_a < token_b { (token_a, token_b) } else { (token_b, token_a) };
            assert!(token_0 != ZERO_ADDRESS.into(), "ZERO ADDRESS");
            (token_0, token_1)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let router = Router::default();
            assert_eq!(router.get(), false);
        }
    }
}
