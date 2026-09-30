use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use crate::instructions::shared::{TransferSolAccounts, TransferSolInstructionData};

pub struct TransferSolWithProgram<'info> {
    pub accounts: TransferSolAccounts<'info>,
    pub instruction_datas: TransferSolInstructionData,
}

impl<'info> TryFrom<(&'info mut [AccountView], &'info [u8])> for TransferSolWithProgram<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info mut [AccountView], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = TransferSolAccounts::try_from(accounts)?;
        let instruction_datas = TransferSolInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> TransferSolWithProgram<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        *self.accounts.payer.try_borrow_mut_lamports()? -= self.instruction_datas.amount;
        *self.accounts.recipient.try_borrow_mut_lamports()? += self.instruction_datas.amount;
        Ok(())
    }
}
