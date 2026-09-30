use counter::state::MutationType;
use pinocchio::{
    account_info::AccountView, cpi::invoke, instruction::AccountMeta, pinocchio::error::ProgramError,
    ProgramResult,
};

pub struct MutateCounterInstructionData {
    pub mutation: MutationType,
}

impl MutateCounterInstructionData {
    pub const LEN: usize = core::mem::size_of::<MutateCounterInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for MutateCounterInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        let mutation =
            MutationType::try_from(data).map_err(|_| ProgramError::InvalidInstructionData)?;

        Ok(Self { mutation })
    }
}

pub struct MutateCounterIxsAccounts<'info> {
    pub maker: &'info AccountView,
    pub counter: &'info AccountView,
    pub counter_program: &'info AccountView,
    pub system_program: &'info AccountView,
}

impl<'info> TryFrom<&'info [AccountView]> for MutateCounterIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountView]) -> Result<Self, Self::Error> {
        let [maker, counter, counter_program, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        Ok(Self {
            maker,
            counter,
            counter_program,
            system_program,
        })
    }
}

pub struct Mutate<'info> {
    pub accounts: MutateCounterIxsAccounts<'info>,
}

impl<'info> TryFrom<&'info [AccountView]> for Mutate<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountView]) -> Result<Self, Self::Error> {
        let accounts = MutateCounterIxsAccounts::try_from(accounts)?;

        Ok(Self { accounts })
    }
}

impl<'info> Mutate<'info> {
    pub fn handler(&mut self, action: MutationType) -> ProgramResult {
        let account_metas: [AccountMeta; 3] = [
            AccountMeta::writable_signer(self.accounts.maker.key()),
            AccountMeta::writable(self.accounts.counter.key()),
            AccountMeta::readonly(self.accounts.system_program.key()),
        ];

        match action {
            MutationType::INCREASE => self.increment(account_metas)?,
            MutationType::DECREASE => self.decrement(account_metas)?,
        }

        Ok(())
    }

    fn increment(&mut self, account_metas: [AccountMeta; 3]) -> ProgramResult {
        let mut instruction_data = [0; 1];
        instruction_data[0] = 1; // Instruction discriminator for Increase
        let instruction = pinocchio::instruction::Instruction {
            program_id: &self.accounts.counter_program.key(),
            accounts: &account_metas,
            data: &instruction_data,
        };

        invoke(&instruction, &[self.accounts.maker, self.accounts.counter])?;
        Ok(())
    }
    fn decrement(&mut self, account_metas: [AccountMeta; 3]) -> ProgramResult {
        let mut instruction_data = [0; 1];
        instruction_data[0] = 2; // Instruction discriminator for Decrease
        let instruction = pinocchio::instruction::Instruction {
            program_id: &self.accounts.counter_program.key(),
            accounts: &account_metas,
            data: &instruction_data,
        };
        invoke(&instruction, &[self.accounts.maker, self.accounts.counter])?;
        Ok(())
    }
}
