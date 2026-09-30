use pinocchio::{error::ProgramError, AccountView, ProgramResult};
use pinocchio_log::log;

use crate::{constants::FAVORITES_SEED, state::Favorites};

pub struct GetPdaIxsAccounts<'info> {
    pub user: &'info AccountView,
    pub favorites: &'info AccountView,
}

impl<'info> TryFrom<&'info mut [AccountView]> for GetPdaIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info mut [AccountView]) -> Result<Self, Self::Error> {
        let [user, favorites] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !user.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // check account is not already initialized
        if favorites.data_len() == 0 {
            return Err(ProgramError::InvalidAccountData);
        }

        //check account has the correct owner
        if !favorites.owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self { user, favorites })
    }
}

pub struct GetPda<'info> {
    pub accounts: GetPdaIxsAccounts<'info>,
}

impl<'info> TryFrom<&'info mut [AccountView]> for GetPda<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info mut [AccountView]) -> Result<Self, Self::Error> {
        let accounts = GetPdaIxsAccounts::try_from(accounts)?;

        Ok(Self { accounts })
    }
}

impl<'info> GetPda<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        let mut favorites_data = self.accounts.favorites.try_borrow_mut()?;
        let favorites = unsafe {
            bytemuck::try_from_bytes_mut::<Favorites>(favorites_data.as_mut())
                .map_err(|_| ProgramError::InvalidAccountData)?
        };

        let seeds = &[FAVORITES_SEED, self.accounts.user.address().as_ref()];
        let (favorites_pubkey, _) = Address::find_program_address(seeds, &crate::ID);

        if self.accounts.favorites.address().ne(&favorites_pubkey) {
            return Err(ProgramError::InvalidAccountData);
        }

        // log account data
        log!(
            "User {}'s favorite number is {}, favorite color is: {}",
            self.accounts.user.address(),
            u64::from_le_bytes(favorites.number),
            bytemuck::from_bytes::<[u8; 50]>(&favorites.color),
        );

        Ok(())
    }
}
