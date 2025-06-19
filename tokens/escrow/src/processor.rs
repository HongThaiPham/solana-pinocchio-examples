use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::instructions::{Instruction, MakeOffer, TakeOffer};
use pinocchio_log::log;

#[inline(always)]
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
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
        Instruction::MakeOffer => {
            log!("Instruction: MakeOffer");
            MakeOffer::try_from((accounts, data))?.handler()
        }
        Instruction::TakeOffer => {
            log!("Instruction: TakeOffer");
            TakeOffer::try_from((accounts, data))?.handler()
        }
    }
}
