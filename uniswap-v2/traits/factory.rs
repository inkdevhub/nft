#[openbrush::trait_definition]
pub trait Factory {
    #[ink(message)]
    fn all_pair_length(&self) -> u128;
}
