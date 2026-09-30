use core::mem::transmute;

use pinocchio::{
    cpi::invoke, error::ProgramError, instruction::AccountMeta, AccountView, ProgramResult,
};
use spl_token_2022::extension::StateWithExtensions;
pub struct MintTokenIxsAccounts<'info> {
    pub mint_authority: &'info AccountView,
    pub mint: &'info AccountView,
    pub to: &'info AccountView,
    // token_account is the associated token account for the mint of to account
    pub token_account: &'info AccountView,
    pub associated_token_program: &'info AccountView,
    pub token_program: &'info AccountView,
    pub system_program: &'info AccountView,
}

impl<'info> TryFrom<&'info mut [AccountView]> for MintTokenIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info mut [AccountView]) -> Result<Self, Self::Error> {
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
        if !mint.owned_by(token_program.address()) {
            return Err(ProgramError::InvalidAccountData);
        }

        // check token_account is writable
        if !token_account.is_writable() {
            return Err(ProgramError::InvalidAccountData);
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
        if !system_program.address().eq(&pinocchio_system::ID) {
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

impl<'info> TryFrom<(&'info mut [AccountView], &'info [u8])> for MintToken<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info mut [AccountView], &'info [u8]),
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
        if self.accounts.token_account.is_data_empty() {
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

        let account_metas: [AccountMeta; 3] = [
            AccountMeta::writable(self.accounts.mint.address()),
            AccountMeta::writable(self.accounts.token_account.address()),
            AccountMeta::readonly_signer(self.accounts.mint_authority.address()),
        ];

        // Instruction data layout:
        // -  [0]: instruction discriminator (1 byte, u8)
        // -  [1..9]: amount (8 bytes, u64)
        // -  [9]: decimals (1 byte, u8)

        let mut instruction_data = [0; 10];
        {
            let binding = self.accounts.mint.try_borrow()?;
            let mint_state = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&binding)
                .map_err(|_| ProgramError::InvalidAccountData)?;

            instruction_data[0] = 14; // Instruction discriminator for MintToChecked
            instruction_data[1..9].copy_from_slice(&self.instruction_datas.amount);
            instruction_data[9] = mint_state.base.decimals;
        }

        let instruction = pinocchio::instruction::Instruction {
            program_id: self.accounts.token_program.address(),
            accounts: &account_metas,
            data: &instruction_data,
        };

        invoke(
            &instruction,
            &[
                self.accounts.mint,
                self.accounts.token_account,
                self.accounts.mint_authority,
            ],
        )?;
        Ok(())
    }
}
