use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_system::instructions::Transfer;

use crate::instructions::shared::{TransferSolAccounts, TransferSolInstructionData};

pub struct TransferSolWithCpi<'info> {
    pub accounts: TransferSolAccounts<'info>,
    pub instruction_datas: TransferSolInstructionData,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for TransferSolWithCpi<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = TransferSolAccounts::try_from(accounts)?;
        let instruction_datas = TransferSolInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> TransferSolWithCpi<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        Transfer {
            from: self.accounts.payer,
            to: self.accounts.recipient,
            lamports: self.instruction_datas.amount,
        }
        .invoke()?;
        Ok(())
    }
}
