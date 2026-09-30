use bytemuck::{Pod, Zeroable};
use pinocchio::{error::ProgramError, AccountView, ProgramResult};
use pinocchio_token::state::{Account, Mint};
pub struct TransferIxsAccounts<'info> {
    pub from: &'info AccountView,
    pub mint: &'info AccountView,
    pub to: &'info AccountView,
    // token_account is the associated token account for the mint of to account
    pub from_token_account: &'info AccountView,
    pub to_token_account: &'info AccountView,
    pub associated_token_program: &'info AccountView,
    pub token_program: &'info AccountView,
    pub system_program: &'info AccountView,
}

impl<'info> TryFrom<&'info mut [AccountView]> for TransferIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info mut [AccountView]) -> Result<Self, Self::Error> {
        let [from, mint, to, from_token_account, to_token_account, associated_token_program, token_program, system_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !from.is_signer() {
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
        if !from_token_account.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }

        if from_token_account.is_data_empty() {
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
            from,
            mint,
            to,
            from_token_account,
            to_token_account,
            associated_token_program,
            token_program,
            system_program,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TransferInstructionData {
    pub amount: [u8; 8],
}

impl TransferInstructionData {
    pub const LEN: usize = core::mem::size_of::<TransferInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for TransferInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        let result = bytemuck::try_from_bytes::<Self>(&data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        Ok(*result)
    }
}

pub struct Transfer<'info> {
    pub accounts: TransferIxsAccounts<'info>,
    pub instruction_datas: TransferInstructionData,
}

impl<'info> TryFrom<(&'info mut [AccountView], &'info [u8])> for Transfer<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info mut [AccountView], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = TransferIxsAccounts::try_from(accounts)?;
        let instruction_datas = TransferInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> Transfer<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        {
            let from_token_account =
                Account::from_account_view(self.accounts.from_token_account)?;
            if from_token_account.amount() < u64::from_le_bytes(self.instruction_datas.amount) {
                return Err(ProgramError::InsufficientFunds);
            }
        }

        if self.accounts.to_token_account.is_data_empty() {
            pinocchio_associated_token_account::instructions::Create {
                account: &self.accounts.to_token_account,
                mint: &self.accounts.mint,
                funding_account: &self.accounts.from,
                system_program: &self.accounts.system_program,
                token_program: &self.accounts.token_program,
                wallet: &self.accounts.to,
            }
            .invoke()?;
        }

        pinocchio_token::instructions::TransferChecked {
            mint: &self.accounts.mint,
            from: &self.accounts.from_token_account,
            to: &self.accounts.to_token_account,
            amount: u64::from_le_bytes(self.instruction_datas.amount),
            authority: &self.accounts.from,
            decimals: Mint::from_account_view(self.accounts.mint)?.decimals(),
        }
        .invoke()?;
        Ok(())
    }
}
