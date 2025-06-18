use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo, cpi::invoke, instruction::AccountMeta, program_error::ProgramError,
    ProgramResult,
};
use pinocchio_log::log;

pub struct CreateCounterIxsAccounts<'info> {
    pub maker: &'info AccountInfo,
    pub counter: &'info AccountInfo,
    pub counter_program: &'info AccountInfo,
    pub system_program: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for CreateCounterIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
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
        let account_metas: [AccountMeta; 3] = [
            AccountMeta::writable_signer(self.accounts.maker.key()),
            AccountMeta::writable(self.accounts.counter.key()),
            AccountMeta::readonly(self.accounts.system_program.key()),
        ];

        // instruction data
        // - [0]: instruction discriminator
        // - [1..9]: initial value
        // - [9]: bump
        let mut instruction_data = [0; 10]; // 1 + 8 + 1 = 10 bytes
        instruction_data[0] = 0; // Instruction discriminator for Create
        instruction_data[1..9].copy_from_slice(&self.instruction_datas.initial_value.to_vec());
        instruction_data[9] = self.instruction_datas.bump;
        log!("Create instruction data: {}", &instruction_data);

        let instruction = pinocchio::instruction::Instruction {
            program_id: &self.accounts.counter_program.key(),
            accounts: &account_metas,
            data: &instruction_data,
        };

        invoke(
            &instruction,
            &[
                self.accounts.maker,
                self.accounts.counter,
                self.accounts.system_program,
            ],
        )?;
        Ok(())
    }
}
