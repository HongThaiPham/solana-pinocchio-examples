use core::mem::transmute;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::find_program_address,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::state::User;

pub struct CreateUserAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub target_account: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for CreateUserAccounts<'info> {
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
        // check target_account is not already initialized
        if target_account.data_len() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        Ok(Self {
            payer,
            target_account,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CreateUserInstructionData {
    pub name: [u8; 64],
}

impl CreateUserInstructionData {
    pub const LEN: usize = core::mem::size_of::<CreateUserInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for CreateUserInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        Ok(unsafe {
            transmute(
                TryInto::<[u8; size_of::<CreateUserInstructionData>()]>::try_into(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
            )
        })
    }
}

pub struct CreateUser<'info> {
    pub accounts: CreateUserAccounts<'info>,
    pub instruction_datas: CreateUserInstructionData,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for CreateUser<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = CreateUserAccounts::try_from(accounts)?;
        let instruction_datas = CreateUserInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> CreateUser<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        let (t_account, bump) = find_program_address(
            &[User::SEED_PREFIX, self.accounts.payer.key().as_ref()],
            &crate::ID,
        );

        if t_account.ne(self.accounts.target_account.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        let bump_binding = [bump];
        let seed = [
            Seed::from(User::SEED_PREFIX),
            Seed::from(self.accounts.payer.key().as_ref()),
            Seed::from(&bump_binding),
        ];
        let signer_seeds = Signer::from(&seed);

        // Create target account
        pinocchio_system::instructions::CreateAccount {
            from: self.accounts.payer,
            to: self.accounts.target_account,
            space: User::LEN as u64,
            lamports: Rent::get()?.minimum_balance(User::LEN),
            owner: &crate::ID,
        }
        .invoke_signed(&[signer_seeds])?;

        // write data to target account

        let mut data = self.accounts.target_account.try_borrow_mut_data()?;
        let user = User::load_mut(data.as_mut())?;
        user.set_inner(self.instruction_datas.name);
        Ok(())
    }
}
