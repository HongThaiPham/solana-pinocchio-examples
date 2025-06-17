pub mod close_user;
pub mod create_user;
use pinocchio::program_error::ProgramError;

#[repr(u8)]
pub enum Instruction {
    CreateUser,
    CloseUser,
}

impl TryFrom<&u8> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(Instruction::CreateUser),
            1 => Ok(Instruction::CloseUser),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
