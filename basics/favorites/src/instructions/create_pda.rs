use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::{constants::FAVORITES_SEED, state::Favorites};

pub struct CreatePdaIxsAccounts<'info> {
    pub user: &'info AccountInfo,
    pub favorites: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for CreatePdaIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [user, favorites, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !user.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // check counter is writable
        if !favorites.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }
        // check counter is not already initialized
        if favorites.data_len() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        Ok(Self { user, favorites })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CreatePdaInstructionData {
    pub number: [u8; 8],
    pub color: [u8; 50],
    pub hobbies: [[u8; 50]; 5],
    pub bump: u8,
}

impl CreatePdaInstructionData {
    pub const LEN: usize = core::mem::size_of::<CreatePdaInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for CreatePdaInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        let result = bytemuck::try_from_bytes::<Self>(&data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        Ok(*result)
    }
}

pub struct CreatePda<'info> {
    pub accounts: CreatePdaIxsAccounts<'info>,
    pub instruction_datas: CreatePdaInstructionData,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for CreatePda<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = CreatePdaIxsAccounts::try_from(accounts)?;
        let instruction_datas = CreatePdaInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> CreatePda<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        let favorites_pubkey = pubkey::create_program_address(
            &[
                FAVORITES_SEED,
                self.accounts.user.key().as_ref(),
                &[self.instruction_datas.bump as u8],
            ],
            &crate::ID,
        )
        .map_err(|_| ProgramError::InvalidSeeds)?;

        if self.accounts.favorites.key() != &favorites_pubkey {
            return Err(ProgramError::InvalidAccountData);
        }

        let bump = [self.instruction_datas.bump as u8];
        let seed = [
            Seed::from(FAVORITES_SEED),
            Seed::from(self.accounts.user.key().as_ref()),
            Seed::from(&bump),
        ];
        let signer_seeds = Signer::from(&seed);

        // Initialize the favorites account
        pinocchio_system::instructions::CreateAccount {
            from: self.accounts.user,
            to: self.accounts.favorites,
            space: Favorites::LEN as u64,
            lamports: Rent::get()?.minimum_balance(Favorites::LEN),
            owner: &crate::ID,
        }
        .invoke_signed(&[signer_seeds])?;

        // // write the initial data to the counter account
        let favorites = unsafe {
            bytemuck::try_from_bytes_mut::<Favorites>(
                self.accounts.favorites.borrow_mut_data_unchecked(),
            )
            .map_err(|_| ProgramError::InvalidAccountData)?
        };

        favorites.set_inner(Favorites {
            number: self.instruction_datas.number,
            color: self.instruction_datas.color,
            hobbies: self.instruction_datas.hobbies,
            bump: self.instruction_datas.bump,
        });

        Ok(())
    }
}
