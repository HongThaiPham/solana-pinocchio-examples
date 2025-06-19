use pinocchio::program_error::ProgramError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowError {
    OfferKeyMismatch,
    TokenAccountMismatch,
}

impl From<EscrowError> for ProgramError {
    fn from(_error: EscrowError) -> Self {
        ProgramError::Custom(6001) // You can use different error codes for different errors
    }
}
