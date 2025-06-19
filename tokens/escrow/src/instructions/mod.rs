pub mod make_offer;
pub use make_offer::*;
pub mod take_offer;
use pinocchio::program_error::ProgramError;
pub use take_offer::*;

#[repr(u8)]
pub enum Instruction {
    MakeOffer,
    TakeOffer,
}

impl TryFrom<&u8> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(Instruction::MakeOffer),
            1 => Ok(Instruction::TakeOffer),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
