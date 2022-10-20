use openbrush::{
    contracts::{
        reentrancy_guard::*,
        traits::{
            ownable::*,
            pausable::*,
            psp22::PSP22Error,
        },
    },
    traits::{
        AccountId,
        Balance,
        Timestamp,
    },
};
use primitive_types::U256;

#[cfg(feature = "std")]
use ink_metadata::layout::{
    CellLayout,
    Layout,
    LayoutKey,
};
use ink_primitives::KeyPtr;
use ink_storage::traits::{
    ExtKeyPtr,
    PackedLayout,
    SpreadAllocate,
    SpreadLayout,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use scale::{
    Decode,
    Encode,
};

#[openbrush::wrapper]
pub type PairRef = dyn Pair;

#[openbrush::trait_definition]
pub trait Pair {
    #[ink(message)]
    fn get_reserves(&self) -> (Balance, Balance, Timestamp);

    #[ink(message)]
    fn price_0_cumulative_last(&self) -> WrappedU256;

    #[ink(message)]
    fn price_1_cumulative_last(&self) -> WrappedU256;

    #[ink(message)]
    fn initialize(&mut self, token_0: AccountId, token_1: AccountId) -> Result<(), PairError>;

    #[ink(message)]
    fn mint(&mut self, to: AccountId) -> Result<Balance, PairError>;

    #[ink(message)]
    fn burn(&mut self, to: AccountId) -> Result<(Balance, Balance), PairError>;

    #[ink(message)]
    fn swap(
        &mut self,
        amount_0_out: Balance,
        amount_1_out: Balance,
        to: AccountId,
    ) -> Result<(), PairError>;

    #[ink(message)]
    fn skim(&mut self, to: AccountId) -> Result<(), PairError>;

    #[ink(message)]
    fn sync(&mut self) -> Result<(), PairError>;

    #[ink(message)]
    fn get_token_0(&self) -> AccountId;

    #[ink(message)]
    fn get_token_1(&self) -> AccountId;

    fn _safe_transfer(
        &mut self,
        token: AccountId,
        to: AccountId,
        value: Balance,
    ) -> Result<(), PairError>;

    fn _mint_fee(&mut self, reserve_0: Balance, reserve_1: Balance) -> Result<bool, PairError>;

    fn _update(
        &mut self,
        balance_0: Balance,
        balance_1: Balance,
        reserve_0: Balance,
        reserve_1: Balance,
    ) -> Result<(), PairError>;

    fn _emit_mint_event(&self, _sender: AccountId, _amount_0: Balance, _amount_1: Balance);
    fn _emit_burn_event(
        &self,
        _sender: AccountId,
        _amount_0: Balance,
        _amount_1: Balance,
        _to: AccountId,
    );
    fn _emit_swap_event(
        &self,
        _sender: AccountId,
        _amount_0_in: Balance,
        _amount_1_in: Balance,
        _amount_0_out: Balance,
        _amount_1_out: Balance,
        _to: AccountId,
    );
    fn _emit_sync_event(&self, reserve_0: Balance, reserve_1: Balance);
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PairError {
    PSP22Error(PSP22Error),
    OwnableError(OwnableError),
    PausableError(PausableError),
    ReentrancyGuardError(ReentrancyGuardError),
    K,
    InsufficientLiquidityMinted,
    InsufficientLiquidityBurned,
    InsufficientOutputAmount,
    InsufficientLiquidity,
    InsufficientInputAmount,
    SafeTransferFailed,
    InvalidTo,
    Overflow,
    Locked,
    SubUnderFlow1,
    SubUnderFlow2,
    SubUnderFlow3,
    SubUnderFlow4,
    SubUnderFlow5,
    SubUnderFlow6,
    SubUnderFlow7,
    SubUnderFlow8,
    SubUnderFlow9,
    SubUnderFlow10,
    SubUnderFlow11,
    SubUnderFlow12,
    SubUnderFlow13,
    SubUnderFlow14,
    MulOverFlow1,
    MulOverFlow2,
    MulOverFlow3,
    MulOverFlow4,
    MulOverFlow5,
    MulOverFlow6,
    MulOverFlow7,
    MulOverFlow8,
    MulOverFlow9,
    MulOverFlow10,
    MulOverFlow11,
    MulOverFlow12,
    MulOverFlow13,
    MulOverFlow14,
    MulOverFlow15,
    DivByZero1,
    DivByZero2,
    DivByZero3,
    DivByZero4,
    DivByZero5,
    AddOverflow1,
}

impl From<OwnableError> for PairError {
    fn from(error: OwnableError) -> Self {
        PairError::OwnableError(error)
    }
}

impl From<PausableError> for PairError {
    fn from(access: PausableError) -> Self {
        PairError::PausableError(access)
    }
}

impl From<PSP22Error> for PairError {
    fn from(error: PSP22Error) -> Self {
        PairError::PSP22Error(error)
    }
}

impl From<ReentrancyGuardError> for PairError {
    fn from(error: ReentrancyGuardError) -> Self {
        PairError::ReentrancyGuardError(error)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct WrappedU256(U256);
impl SpreadLayout for WrappedU256 {
    const FOOTPRINT: u64 = 4;
    const REQUIRES_DEEP_CLEAN_UP: bool = true;
    fn pull_spread(ptr: &mut ink_primitives::KeyPtr) -> Self {
        let slice: [u64; 4] = SpreadLayout::pull_spread(ptr);
        Self(U256(slice))
    }

    fn push_spread(&self, ptr: &mut ink_primitives::KeyPtr) {
        SpreadLayout::push_spread(&self.0 .0, ptr);
    }

    fn clear_spread(&self, ptr: &mut ink_primitives::KeyPtr) {
        SpreadLayout::clear_spread(&self.0 .0, ptr);
    }
}

impl PackedLayout for WrappedU256 {
    fn pull_packed(&mut self, at: &ink_primitives::Key) {
        self.0 .0.pull_packed(at);
    }
    fn push_packed(&self, at: &ink_primitives::Key) {
        self.0 .0.push_packed(at);
    }
    fn clear_packed(&self, at: &ink_primitives::Key) {
        self.0 .0.clear_packed(at);
    }
}

impl SpreadAllocate for WrappedU256 {
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        ptr.next_for::<WrappedU256>();
        WrappedU256::default()
    }
}

#[cfg(feature = "std")]
impl StorageLayout for WrappedU256 {
    fn layout(key_ptr: &mut KeyPtr) -> Layout {
        Layout::Cell(CellLayout::new::<WrappedU256>(LayoutKey::from(
            key_ptr.advance_by(1),
        )))
    }
}

impl From<WrappedU256> for U256 {
    fn from(value: WrappedU256) -> Self {
        value.0
    }
}

impl From<U256> for WrappedU256 {
    fn from(value: U256) -> Self {
        WrappedU256(value)
    }
}

macro_rules! construct_from {
    ( $( $type:ident ),* ) => {
        $(
            impl TryFrom<WrappedU256> for $type {
                type Error = &'static str;
                #[inline]
                fn try_from(value: WrappedU256) -> Result<Self, Self::Error> {
                    Self::try_from(value.0)
                }
            }

            impl From<$type> for WrappedU256 {
                fn from(value: $type) -> WrappedU256 {
                    WrappedU256(U256::from(value))
                }
            }
        )*
    };
}

construct_from!(u8, u16, u32, u64, usize, i8, i16, i32, i64);
