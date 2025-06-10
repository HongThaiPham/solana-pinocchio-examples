use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use pinocchio_log::log;

use crate::instructions::{create_token, mint_token, Instruction};

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
        Instruction::CreateToken => {
            log!("Instruction::CreateToken");
            create_token::CreateToken::try_from((accounts, data))?.handler()
        }
        Instruction::MintToken => {
            log!("Instruction::MintToken");
            // Handle MintToken instruction here
            mint_token::MintToken::try_from((accounts, data))?.handler()
        }
    }
}
