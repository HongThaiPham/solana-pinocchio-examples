use core::{mem::MaybeUninit, slice::from_raw_parts};

use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    ProgramResult,
};
pub struct CreateTokenIxsAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub mint: &'info AccountInfo,
    pub p_token_program: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for CreateTokenIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, mint, p_token_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // check counter is writable
        if !mint.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // check counter is not already initialized
        if mint.data_len() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // check p_token_program is a valid token program
        if !p_token_program.executable() {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(Self {
            payer,
            mint,
            p_token_program,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CreateTokenInstructionData {
    pub token_decimals: u8,
    pub mint_authority: [u8; 32],
    pub freeze_authority: [u8; 32],
}

impl CreateTokenInstructionData {
    pub const LEN: usize = core::mem::size_of::<CreateTokenInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for CreateTokenInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        let result = bytemuck::try_from_bytes::<Self>(&data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        Ok(*result)
    }
}

pub struct CreateToken<'info> {
    pub accounts: CreateTokenIxsAccounts<'info>,
    pub instruction_datas: CreateTokenInstructionData,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for CreateToken<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = CreateTokenIxsAccounts::try_from(accounts)?;
        let instruction_datas = CreateTokenInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> CreateToken<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        pinocchio_token::instructions::InitializeMint2 {
            mint: self.accounts.mint,
            decimals: self.instruction_datas.token_decimals,
            mint_authority: &self.instruction_datas.mint_authority,
            freeze_authority: Option::Some(&self.instruction_datas.freeze_authority),
        }
        .invoke()?;
        Ok(())
    }
}
