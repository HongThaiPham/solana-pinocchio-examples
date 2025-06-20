pub mod create_token;
pub use create_token::*;
pub mod mint_token;
pub use mint_token::*;
pub mod transfer;
use pinocchio::program_error::ProgramError;
pub use transfer::*;

#[repr(u8)]
pub enum Instruction {
    CreateToken,
    MintToken,
    Transfer,
}

impl TryFrom<&u8> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(Instruction::CreateToken),
            1 => Ok(Instruction::MintToken),
            2 => Ok(Instruction::Transfer),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
