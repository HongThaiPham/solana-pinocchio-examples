use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::find_program_address,
    ProgramResult,
};

use crate::state::User;

pub struct CloseUserAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub target_account: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for CloseUserAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, target_account, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // check target_account is writable
        if !target_account.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }
        // check target_account is initialized
        if target_account.data_len() == 0 {
            return Err(ProgramError::UninitializedAccount);
        }

        if !target_account.is_owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self {
            payer,
            target_account,
        })
    }
}

pub struct CloseUser<'info> {
    pub accounts: CloseUserAccounts<'info>,
}

impl<'info> TryFrom<&'info [AccountInfo]> for CloseUser<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = CloseUserAccounts::try_from(accounts)?;

        Ok(Self { accounts })
    }
}

impl<'info> CloseUser<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        let (t_account, _) = find_program_address(
            &[User::SEED_PREFIX, self.accounts.payer.key().as_ref()],
            &crate::ID,
        );

        if t_account.ne(self.accounts.target_account.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        self.close_program_account(self.accounts.target_account, self.accounts.payer)?;

        Ok(())
    }

    fn close_program_account(
        &self,
        account: &AccountInfo,
        destination: &AccountInfo,
    ) -> ProgramResult {
        {
            let mut data = account.try_borrow_mut_data()?;
            data[0] = 0xff;
        }

        *destination.try_borrow_mut_lamports()? += *account.try_borrow_lamports()?;
        account.realloc(1, true)?;
        account.close()
    }
}
