use pinocchio::program_error::ProgramError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CounterError {
    Overflow,
}

impl From<CounterError> for ProgramError {
    fn from(_error: CounterError) -> Self {
        ProgramError::Custom(6001) // You can use different error codes for different errors
    }
}
