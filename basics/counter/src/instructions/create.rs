use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::{constants::COUNTER_SEED, state::Counter};

pub struct CreateCounterIxsAccounts<'info> {
    pub maker: &'info AccountInfo,
    pub counter: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for CreateCounterIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [maker, counter, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !maker.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // check counter is writable
        if !counter.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }
        // check counter is not already initialized
        if counter.data_len() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        Ok(Self { maker, counter })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CreateCounterInstructionData {
    pub initial_value: [u8; 8],
    pub bump: u8,
}

impl CreateCounterInstructionData {
    pub const LEN: usize = core::mem::size_of::<CreateCounterInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for CreateCounterInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        let result = bytemuck::try_from_bytes::<Self>(&data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        Ok(*result)
    }
}

pub struct Create<'info> {
    pub accounts: CreateCounterIxsAccounts<'info>,
    pub instruction_datas: CreateCounterInstructionData,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for Create<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = CreateCounterIxsAccounts::try_from(accounts)?;
        let instruction_datas = CreateCounterInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> Create<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        let counter_pubkey = pubkey::create_program_address(
            &[COUNTER_SEED, &[self.instruction_datas.bump as u8]],
            &crate::ID,
        )
        .map_err(|_| ProgramError::InvalidSeeds)?;

        if self.accounts.counter.key() != &counter_pubkey {
            return Err(ProgramError::InvalidAccountData);
        }
        let bump = [self.instruction_datas.bump as u8];
        let seed = [Seed::from(COUNTER_SEED), Seed::from(&bump)];
        let signer_seeds = Signer::from(&seed);

        // Initialize the counter account
        pinocchio_system::instructions::CreateAccount {
            from: self.accounts.maker,
            to: self.accounts.counter,
            space: Counter::LEN as u64,
            lamports: Rent::get()?.minimum_balance(Counter::LEN),
            owner: &crate::ID,
        }
        .invoke_signed(&[signer_seeds])?;

        // write the initial data to the counter account
        let counter = unsafe {
            bytemuck::try_from_bytes_mut::<Counter>(
                self.accounts.counter.borrow_mut_data_unchecked(),
            )
            .map_err(|_| ProgramError::InvalidAccountData)?
        };

        counter.set_inner(Counter {
            count: self.instruction_datas.initial_value,
        });
        Ok(())
    }
}
