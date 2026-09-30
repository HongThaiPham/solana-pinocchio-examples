use core::mem::transmute;

use pinocchio::{
    AccountView,
    Address,
    error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use spl_token_2022::state::PackedSizeOf;
pub struct CreateTokenIxsAccounts<'info> {
    pub payer: &'info AccountView,
    pub mint: &'info AccountView,
    pub token_program: &'info AccountView,
}

impl<'info> TryFrom<&'info mut [AccountView]> for CreateTokenIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info mut [AccountView]) -> Result<Self, Self::Error> {
        let [payer, mint, token_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // check mint is writable
        if !mint.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // check mint is uninitialized
        if mint.data_len() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // check token_program is a valid token program
        if !token_program.executable() {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(Self {
            payer,
            mint,
            token_program: token_program,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CreateTokenInstructionData {
    pub token_name: [u8; 32],
    pub token_decimals: u8,
}

impl CreateTokenInstructionData {
    pub const LEN: usize = core::mem::size_of::<CreateTokenInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for CreateTokenInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        Ok(unsafe {
            transmute(
                TryInto::<[u8; size_of::<CreateTokenInstructionData>()]>::try_into(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
            )
        })
    }
}

pub struct CreateToken<'info> {
    pub accounts: CreateTokenIxsAccounts<'info>,
    pub instruction_datas: CreateTokenInstructionData,
}

impl<'info> TryFrom<(&'info mut [AccountView], &'info [u8])> for CreateToken<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info mut [AccountView], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = CreateTokenIxsAccounts::try_from(accounts)?;
        let instruction_datas = CreateTokenInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> CreateToken<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        pinocchio_system::instructions::CreateAccount {
            from: self.accounts.payer,
            to: self.accounts.mint,
            space: spl_token_2022::state::Mint::SIZE_OF as u64,
            lamports: Rent::get()?.minimum_balance(spl_token_2022::state::Mint::SIZE_OF),
            owner: self.accounts.token_program.address(),
        }
        .invoke()?;

        pinocchio_token::instructions::InitializeMint2::new(
            self.accounts.mint,
            self.instruction_datas.token_decimals,
            self.accounts.payer.address(),
            Some(self.accounts.payer.address()),
        )
        .invoke_with_program(self.accounts.token_program.address())?;

        Ok(())
    }
}
