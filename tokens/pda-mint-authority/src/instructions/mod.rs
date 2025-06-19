pub mod create_token;
pub use create_token::*;
pub mod init;
pub use init::*;
pub mod mint_token;
pub use mint_token::*;
use pinocchio::program_error::ProgramError;

#[repr(u8)]
pub enum Instruction {
    InitMintAuthority,
    CreateToken,
    MintToken,
}

impl TryFrom<&u8> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(Instruction::InitMintAuthority),
            1 => Ok(Instruction::CreateToken),
            2 => Ok(Instruction::MintToken),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
