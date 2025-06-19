use core::mem::transmute;

use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_token::state::Mint;

use crate::state::MintAuthority;
pub struct CreateTokenIxsAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub mint: &'info AccountInfo,
    pub mint_authority: &'info AccountInfo,
    pub token_program: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for CreateTokenIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, mint, mint_authority, token_program, _] = accounts else {
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

        // check token_program is a valid token program
        if !token_program.executable() {
            return Err(ProgramError::IncorrectProgramId);
        }

        if mint_authority.data_is_empty() {
            return Err(ProgramError::UninitializedAccount);
        }
        if !mint_authority.is_owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        let (mint_authority_key, _) =
            find_program_address(&[MintAuthority::SEED_PREFIX], &crate::ID);

        if mint_authority_key.ne(mint_authority.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self {
            payer,
            mint,
            mint_authority,
            token_program,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CreateTokenInstructionData {
    pub token_decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: Pubkey,
}

impl CreateTokenInstructionData {
    pub const LEN: usize = core::mem::size_of::<CreateTokenInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for CreateTokenInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        Ok(unsafe {
            transmute(
                TryInto::<[u8; size_of::<CreateTokenInstructionData>()]>::try_into(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
            )
        })
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
        pinocchio_system::instructions::CreateAccount {
            from: self.accounts.payer,
            to: self.accounts.mint,
            space: Mint::LEN as u64,
            lamports: Rent::get()?.minimum_balance(Mint::LEN),
            owner: &self.accounts.token_program.key(),
        }
        .invoke()?;

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
