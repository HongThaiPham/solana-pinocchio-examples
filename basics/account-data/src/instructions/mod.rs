pub mod create;

pub use create::*;

use pinocchio::program_error::ProgramError;

#[repr(u8)]
pub enum Instruction {
    Create,
}

impl TryFrom<&u8> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(Instruction::Create),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
