use crate::traits::rewarder::errors::RewarderError;
use openbrush::contracts::{
    ownable::*,
    traits::psp22::PSP22Error,
};

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FarmingError {
    OwnableError(OwnableError),
    PSP22Error(PSP22Error),
    RewarderError(RewarderError),
    DuplicateLPToken,
    PoolNotFound,
    UserNotFound,
    ZeroWithdrawal,
    LpTokenNotFound,
    LpSupplyIsZero,
    BlockNumberLowerThanOriginBlock,
    CastTou128Error1,
    CastTou128Error2,
    CastTou128Error3,
    CastTou128Error4,
    CastTou128Error5,
    CastTou128Error6,
    CastTou128Error7,
    CastToi128Error,
    CastToi128Error2,
    CastToi128Error3,
    CastToi128Error4,
    CastToi128Error5,
    CastToi128Error6,
    CastToi128Error7,
    RewarderNotFound,
    SubUnderflow1,
    SubUnderflow2,
    SubUnderflow3,
    SubUnderflow4,
    SubUnderflow5,
    SubUnderflow6,
    SubUnderflow7,
    SubUnderflow8,
    SubUnderflow9,
    SubUnderflow10,
    AddOverflow1,
    AddOverflow2,
    AddOverflow3,
    AddOverflow4,
    AddOverflow5,
    AddOverflow6,
    AddOverflow7,
    AddOverflow8,
    AddOverflow9,
    AddOverflow10,
    AddOverflow11,
    AddOverflow12,
    AddOverflow13,
    AddOverflow14,
    AddOverflow15,
    AddOverflow16,
    MulOverflow1,
    MulOverflow2,
    MulOverflow3,
    MulOverflow4,
    MulOverflow5,
    MulOverflow6,
    MulOverflow7,
    MulOverflow8,
    MulOverflow9,
    MulOverflow10,
    MulOverflow11,
    MulOverflow12,
    MulOverflow13,
    PowOverflow1,
    PowOverflow2,
    DivByZero1,
    DivByZero2,
    DivByZero3,
}

impl From<OwnableError> for FarmingError {
    fn from(error: OwnableError) -> Self {
        FarmingError::OwnableError(error)
    }
}

impl From<PSP22Error> for FarmingError {
    fn from(error: PSP22Error) -> Self {
        FarmingError::PSP22Error(error)
    }
}

impl From<RewarderError> for FarmingError {
    fn from(error: RewarderError) -> Self {
        FarmingError::RewarderError(error)
    }
}
