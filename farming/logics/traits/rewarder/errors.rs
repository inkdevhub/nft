use openbrush::contracts::traits::psp22::PSP22Error;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RewarderError {
    PSP22Error(PSP22Error),
    CallerIsNotMasterChef,
    MulOverflow1,
    MulOverflow2,
}

impl From<PSP22Error> for RewarderError {
    fn from(error: PSP22Error) -> Self {
        RewarderError::PSP22Error(error)
    }
}
