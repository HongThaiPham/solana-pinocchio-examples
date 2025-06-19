use core::mem::transmute;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::find_program_address,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::state::MintAuthority;

pub struct InitMintAuthorityAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub mint_authority: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for InitMintAuthorityAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, mint_authority, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !mint_authority.data_is_empty() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let (mint_authority_key, _) =
            find_program_address(&[MintAuthority::SEED_PREFIX], &crate::ID);

        if mint_authority_key.ne(mint_authority.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self {
            payer,
            mint_authority,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InitMintAuthorityInstructionData {
    pub bump: u8,
}

impl InitMintAuthorityInstructionData {
    pub const LEN: usize = core::mem::size_of::<InitMintAuthorityInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for InitMintAuthorityInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        Ok(unsafe {
            transmute(
                TryInto::<[u8; size_of::<InitMintAuthorityInstructionData>()]>::try_into(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
            )
        })
    }
}

pub struct InitMintAuthority<'info> {
    pub accounts: InitMintAuthorityAccounts<'info>,
    pub instruction_datas: InitMintAuthorityInstructionData,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for InitMintAuthority<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = InitMintAuthorityAccounts::try_from(accounts)?;
        let instruction_datas = InitMintAuthorityInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> InitMintAuthority<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        let bump_binding = [self.instruction_datas.bump];
        let seed = [
            Seed::from(MintAuthority::SEED_PREFIX),
            Seed::from(&bump_binding),
        ];
        let signer_seeds = Signer::from(&seed);

        pinocchio_system::instructions::CreateAccount {
            from: self.accounts.payer,
            to: self.accounts.mint_authority,
            space: MintAuthority::LEN as u64,
            lamports: Rent::get()?.minimum_balance(MintAuthority::LEN),
            owner: &crate::ID,
        }
        .invoke_signed(&[signer_seeds])?;

        let mut data = self.accounts.mint_authority.try_borrow_mut_data()?;
        let mint_authority = MintAuthority::load_mut(data.as_mut())?;
        mint_authority.set_inner(self.instruction_datas.bump);
        Ok(())
    }
}
