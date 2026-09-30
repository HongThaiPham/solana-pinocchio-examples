use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};

use crate::instructions::{Create, Instruction};
use pinocchio_log::log;

#[inline(always)]
pub fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    // Validate program ID
    if _program_id != &crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match Instruction::try_from(discriminator)? {
        Instruction::Create => {
            log!("Instruction: Create");
            Create::try_from((accounts, data))?.handler()
        }
    }
}
