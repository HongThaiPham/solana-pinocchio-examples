use core::mem::transmute;

use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

pub struct TransferSolAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub recipient: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for TransferSolAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, recipient, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // check target_account is writable
        if !recipient.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }

        if !recipient.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self { payer, recipient })
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TransferSolInstructionData {
    pub amount: u64,
}

impl TransferSolInstructionData {
    pub const LEN: usize = core::mem::size_of::<TransferSolInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for TransferSolInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        Ok(unsafe {
            transmute(
                TryInto::<[u8; size_of::<TransferSolInstructionData>()]>::try_into(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
            )
        })
    }
}
