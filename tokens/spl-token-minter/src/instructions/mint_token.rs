use core::mem::transmute;

use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_token::state::{Mint, TokenAccount};
pub struct MintTokenIxsAccounts<'info> {
    pub mint_authority: &'info AccountInfo,
    pub mint: &'info AccountInfo,
    pub to: &'info AccountInfo,
    // token_account is the associated token account for the mint of to account
    pub token_account: &'info AccountInfo,
    pub associated_token_program: &'info AccountInfo,
    pub token_program: &'info AccountInfo,
    pub system_program: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for MintTokenIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [mint_authority, mint, to, token_account, associated_token_program, token_program, system_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !mint_authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // check mint is writable
        if !mint.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }

        // check mint owner is token program
        if !mint.is_owned_by(token_program.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        // check mint_authority is signer
        let mint_info = Mint::from_account_info(mint)?;

        if mint_info
            .mint_authority()
            .is_some_and(|authority| !mint_authority.key().eq(authority))
        {
            return Err(ProgramError::IncorrectAuthority);
        }

        // check token_account is writable
        if !token_account.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }

        if !token_account.data_is_empty() {
            let token_account_info = TokenAccount::from_account_info(token_account)?;
            if !token_account_info.owner().eq(to.key()) {
                return Err(ProgramError::InvalidAccountData);
            }
        }

        // check token_program is a valid token program
        if !token_program.executable() {
            return Err(ProgramError::IncorrectProgramId);
        }

        // check associated_token_program is a valid associated token program
        if !associated_token_program.executable() {
            return Err(ProgramError::IncorrectProgramId);
        }

        // //check system_program is a valid system program
        if !system_program.key().eq(&pinocchio_system::ID) {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(Self {
            mint_authority,
            mint,
            to,
            token_account,
            associated_token_program,
            token_program,
            system_program,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MintTokenInstructionData {
    pub amount: [u8; 8],
}

impl MintTokenInstructionData {
    pub const LEN: usize = core::mem::size_of::<MintTokenInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for MintTokenInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        Ok(unsafe {
            transmute(
                TryInto::<[u8; size_of::<MintTokenInstructionData>()]>::try_into(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
            )
        })
    }
}

pub struct MintToken<'info> {
    pub accounts: MintTokenIxsAccounts<'info>,
    pub instruction_datas: MintTokenInstructionData,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for MintToken<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = MintTokenIxsAccounts::try_from(accounts)?;
        let instruction_datas = MintTokenInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> MintToken<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        if self.accounts.token_account.data_is_empty() {
            pinocchio_associated_token_account::instructions::Create {
                account: &self.accounts.token_account,
                mint: &self.accounts.mint,
                funding_account: &self.accounts.mint_authority,
                system_program: &self.accounts.system_program,
                token_program: &self.accounts.token_program,
                wallet: &self.accounts.to,
            }
            .invoke()?;
        }

        pinocchio_token::instructions::MintTo {
            mint: &self.accounts.mint,
            account: &self.accounts.token_account,
            amount: u64::from_le_bytes(self.instruction_datas.amount),
            mint_authority: &self.accounts.mint_authority,
        }
        .invoke()?;
        Ok(())
    }
}
