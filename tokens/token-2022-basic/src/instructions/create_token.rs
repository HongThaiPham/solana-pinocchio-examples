use core::mem::transmute;

use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke,
    instruction::AccountMeta,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use spl_token_2022::state::PackedSizeOf;
pub struct CreateTokenIxsAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub mint: &'info AccountInfo,
    pub token_program: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for CreateTokenIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
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

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for CreateToken<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
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
            owner: &self.accounts.token_program.key(),
        }
        .invoke()?;

        let account_metas: [AccountMeta; 1] = [AccountMeta::writable(self.accounts.mint.key())];

        // Instruction data layout:
        // -  [0]: instruction discriminator (1 byte, u8)
        // -  [1]: decimals (1 byte, u8)
        // -  [2..34]: mint_authority (32 bytes, Pubkey)
        // -  [34]: freeze_authority presence flag (1 byte, u8)
        // -  [35..67]: freeze_authority (optional, 32 bytes, Pubkey)

        let mut instruction_data = [0; 67];
        instruction_data[0] = 20; // Instruction discriminator for InitializeMint2
        instruction_data[1] = self.instruction_datas.token_decimals;
        instruction_data[2..34].copy_from_slice(self.accounts.payer.key());
        instruction_data[34] = 1; // Freeze authority presence flag (1 byte, u8)

        instruction_data[35..67].copy_from_slice(self.accounts.payer.key());

        let instruction = pinocchio::instruction::Instruction {
            program_id: &self.accounts.token_program.key(),
            accounts: &account_metas,
            data: &instruction_data,
        };

        invoke(&instruction, &[self.accounts.mint])?;

        Ok(())
    }
}
