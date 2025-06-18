pub mod create;

pub use create::*;

pub mod mutate;
pub use mutate::*;

use pinocchio::program_error::ProgramError;

#[repr(u8)]
pub enum Instruction {
    Create,
    Increase,
    Decrease,
}

impl TryFrom<&u8> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(Instruction::Create),
            1 => Ok(Instruction::Increase),
            2 => Ok(Instruction::Decrease),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
