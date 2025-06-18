use counter::state::MutationType;
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use pinocchio_log::log;

use crate::instructions::{Create, Instruction, Mutate};

#[inline(always)]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Validate program ID
    if program_id != &crate::ID {
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
        Instruction::Increase => {
            log!("Instruction: Increase");
            Mutate::try_from(accounts)?.handler(MutationType::INCREASE)
        }
        Instruction::Decrease => {
            log!("Instruction: Decrease");
            Mutate::try_from(accounts)?.handler(MutationType::DECREASE)
        }
    }
}
