use core::mem::transmute;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::find_program_address,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_token::state::Mint;

use crate::state::Offer;

pub struct MakeOfferAccounts<'info> {
    pub maker: &'info AccountInfo,
    pub token_mint_a: &'info AccountInfo,
    pub token_mint_b: &'info AccountInfo,
    pub maker_ata_a: &'info AccountInfo,
    pub offer: &'info AccountInfo,
    pub vault: &'info AccountInfo,
    pub token_program: &'info AccountInfo,
    pub system_program: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for MakeOfferAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [maker, token_mint_a, token_mint_b, maker_ata_a, offer, vault, token_program, system_program, _] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !maker.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !token_mint_a.is_owned_by(token_program.key()) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !token_mint_b.is_owned_by(token_program.key()) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !maker_ata_a.is_owned_by(token_program.key()) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if maker_ata_a
            .data_len()
            .ne(&pinocchio_token::state::TokenAccount::LEN)
        {
            return Err(ProgramError::InvalidAccountData);
        }

        let (maker_ata_a_address, _) = find_program_address(
            &[maker.key(), token_program.key(), token_mint_a.key()],
            &pinocchio_associated_token_account::ID,
        );

        if maker_ata_a_address.ne(maker_ata_a.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        if !offer.data_is_empty() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if !offer.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }

        let (vault_address, _) = find_program_address(
            &[offer.key(), token_program.key(), token_mint_a.key()],
            &pinocchio_associated_token_account::ID,
        );

        if vault_address.ne(vault.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        if !vault.data_is_empty() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if !vault.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self {
            maker,
            token_mint_a,
            token_mint_b,
            offer,
            vault,
            maker_ata_a,
            token_program,
            system_program,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MakeOfferInstructionData {
    pub id: [u8; 8],
    pub token_a_amount: [u8; 8],
    pub token_b_wanted_amount: [u8; 8],
    pub bump: u8,
}

impl MakeOfferInstructionData {
    pub const LEN: usize = core::mem::size_of::<MakeOfferInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for MakeOfferInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        Ok(unsafe {
            transmute(
                TryInto::<[u8; size_of::<MakeOfferInstructionData>()]>::try_into(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
            )
        })
    }
}

pub struct MakeOffer<'info> {
    pub accounts: MakeOfferAccounts<'info>,
    pub instruction_datas: MakeOfferInstructionData,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for MakeOffer<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = MakeOfferAccounts::try_from(accounts)?;
        let instruction_datas = MakeOfferInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> MakeOffer<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        let (offer_address, bump) = find_program_address(
            &[
                Offer::SEED_PREFIX,
                self.accounts.maker.key().as_ref(),
                &self.instruction_datas.id,
            ],
            &crate::ID,
        );

        if offer_address.ne(self.accounts.offer.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        let bump_binding = [bump];
        let id_binding = self.instruction_datas.id;
        let seed = [
            Seed::from(Offer::SEED_PREFIX),
            Seed::from(self.accounts.maker.key().as_ref()),
            Seed::from(&id_binding),
            Seed::from(&bump_binding),
        ];
        let signer_seeds = Signer::from(&seed);

        // Create offer account
        pinocchio_system::instructions::CreateAccount {
            from: self.accounts.maker,
            to: self.accounts.offer,
            space: Offer::LEN as u64,
            lamports: Rent::get()?.minimum_balance(Offer::LEN),
            owner: &crate::ID,
        }
        .invoke_signed(&[signer_seeds])?;

        {
            let mut data = self.accounts.offer.try_borrow_mut_data()?;
            let offer = Offer::load_mut(data.as_mut())?;
            offer.set_inner(
                self.instruction_datas.id,
                *self.accounts.maker.key(),
                *self.accounts.token_mint_a.key(),
                *self.accounts.token_mint_b.key(),
                self.instruction_datas.token_b_wanted_amount,
                bump,
            );
        }

        {
            pinocchio_associated_token_account::instructions::Create {
                account: self.accounts.vault,
                funding_account: self.accounts.maker,
                mint: self.accounts.token_mint_a,
                token_program: self.accounts.token_program,
                system_program: self.accounts.system_program,
                wallet: self.accounts.offer,
            }
            .invoke()?;

            pinocchio_token::instructions::TransferChecked {
                from: self.accounts.maker_ata_a,
                to: self.accounts.vault,
                authority: self.accounts.maker,
                mint: self.accounts.token_mint_a,
                amount: u64::from_le_bytes(self.instruction_datas.token_a_amount),
                decimals: Mint::from_account_info(self.accounts.token_mint_a)?.decimals(),
            }
            .invoke()?;
        }
        Ok(())
    }
}
