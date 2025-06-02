use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::{
    state::{AddressInfo, CreateAddressInfoAccounts, CreateAddressInfoInstructionData},
    util::load_acc_mut_unchecked,
};

pub struct Create<'info> {
    pub accounts: CreateAddressInfoAccounts<'info>,
    pub instruction_datas: CreateAddressInfoInstructionData,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for Create<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = CreateAddressInfoAccounts::try_from(accounts)?;
        let instruction_datas = CreateAddressInfoInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> Create<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        // Create address info account
        pinocchio_system::instructions::CreateAccount {
            from: self.accounts.payer,
            to: self.accounts.address_info,
            space: AddressInfo::LEN as u64,
            lamports: Rent::get()?.minimum_balance(AddressInfo::LEN),
            owner: &crate::ID,
        }
        .invoke()?;

        // write data to address info account
        let address_info_state = unsafe {
            load_acc_mut_unchecked::<AddressInfo>(
                self.accounts.address_info.borrow_mut_data_unchecked(),
            )
        }?;

        address_info_state.set_inner(AddressInfo {
            name: self.instruction_datas.name,
            house_number: self.instruction_datas.house_number,
            street: self.instruction_datas.street,
            city: self.instruction_datas.city,
        });

        Ok(())
    }
}
