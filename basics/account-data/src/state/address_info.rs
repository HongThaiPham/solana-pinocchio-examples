use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

pub struct CreateAddressInfoAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub address_info: &'info AccountInfo,
}

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct AddressInfo {
    pub name: [u8; 50],
    pub house_number: u8,
    pub street: [u8; 50],
    pub city: [u8; 50],
}

impl AddressInfo {
    pub const LEN: usize = core::mem::size_of::<AddressInfo>();

    pub fn set_inner(&mut self, data: Self) {
        self.name = data.name;
        self.house_number = data.house_number;
        self.street = data.street;
        self.city = data.city;
    }
}

impl<'info> TryFrom<&'info [AccountInfo]> for CreateAddressInfoAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, address_info, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // check address_info is writable
        if !address_info.is_signer() {
            return Err(ProgramError::InvalidAccountData);
        }
        // check address_info is not already initialized
        if address_info.data_len() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        Ok(Self {
            payer,
            address_info,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CreateAddressInfoInstructionData {
    pub name: [u8; 50],
    pub house_number: u8,
    pub street: [u8; 50],
    pub city: [u8; 50],
}

impl CreateAddressInfoInstructionData {
    pub const LEN: usize = core::mem::size_of::<CreateAddressInfoInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for CreateAddressInfoInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        let result = bytemuck::try_from_bytes::<Self>(&data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        Ok(*result)
    }
}
