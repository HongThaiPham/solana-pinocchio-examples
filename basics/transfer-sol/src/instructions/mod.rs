pub mod shared;
pub mod transfer_sol_with_cpi;
pub mod transfer_sol_with_program;
use pinocchio::program_error::ProgramError;

#[repr(u8)]
pub enum Instruction {
    TransferSolWithProgram,
    TransferSolWithCpi,
}

impl TryFrom<&u8> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(Instruction::TransferSolWithProgram),
            1 => Ok(Instruction::TransferSolWithCpi),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
