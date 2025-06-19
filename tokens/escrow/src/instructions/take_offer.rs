use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::find_program_address,
    ProgramResult,
};
use pinocchio_token::state::{Mint, TokenAccount};

use crate::state::Offer;

pub struct TakeOfferAccounts<'info> {
    pub taker: &'info AccountInfo,
    pub token_mint_a: &'info AccountInfo,
    pub token_mint_b: &'info AccountInfo,
    pub taker_ata_a: &'info AccountInfo,
    pub taker_ata_b: &'info AccountInfo,
    pub maker: &'info AccountInfo,
    pub maker_ata_b: &'info AccountInfo,
    pub offer: &'info AccountInfo,
    pub vault: &'info AccountInfo,
    pub token_program: &'info AccountInfo,
    pub system_program: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for TakeOfferAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [taker, token_mint_a, token_mint_b, taker_ata_a, taker_ata_b, maker, maker_ata_b, offer, vault, token_program, system_program, _] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !taker.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !token_mint_a.is_owned_by(token_program.key()) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !token_mint_b.is_owned_by(token_program.key()) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !taker_ata_a.is_owned_by(token_program.key()) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if taker_ata_a
            .data_len()
            .ne(&pinocchio_token::state::TokenAccount::LEN)
        {
            return Err(ProgramError::InvalidAccountData);
        }

        let (taker_ata_a_address, _) = find_program_address(
            &[taker.key(), token_program.key(), token_mint_a.key()],
            &pinocchio_associated_token_account::ID,
        );

        if taker_ata_a_address.ne(taker_ata_a.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        if offer.data_is_empty() {
            return Err(ProgramError::UninitializedAccount);
        }

        if !offer.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }

        let data = offer.try_borrow_data()?;
        let offer_data = Offer::load(&data)?;
        let (offer_address, bump) = find_program_address(
            &[Offer::SEED_PREFIX, maker.key().as_ref(), &offer_data.id],
            &crate::ID,
        );

        if !offer.is_owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if offer_address.ne(offer.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        if offer_data.token_mint_a.ne(token_mint_a.key())
            || offer_data.token_mint_b.ne(token_mint_b.key())
            || offer_data.bump != bump
        {
            return Err(ProgramError::InvalidAccountData);
        }

        let (vault_address, _) = find_program_address(
            &[offer.key(), token_program.key(), token_mint_a.key()],
            &pinocchio_associated_token_account::ID,
        );

        if vault_address.ne(vault.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        if vault.data_is_empty() {
            return Err(ProgramError::UninitializedAccount);
        }

        if !vault.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self {
            taker,
            token_mint_a,
            token_mint_b,
            offer,
            vault,
            taker_ata_a,
            taker_ata_b,
            maker,
            maker_ata_b,
            token_program,
            system_program,
        })
    }
}

pub struct TakeOffer<'info> {
    pub accounts: TakeOfferAccounts<'info>,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for TakeOffer<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, _data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = TakeOfferAccounts::try_from(accounts)?;

        Ok(Self { accounts })
    }
}

impl<'info> TakeOffer<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        let data = self.accounts.offer.try_borrow_data()?;
        let offer = Offer::load(&data)?;

        {
            if self.accounts.maker_ata_b.data_is_empty() {
                pinocchio_associated_token_account::instructions::Create {
                    account: self.accounts.maker_ata_b,
                    funding_account: self.accounts.taker,
                    mint: self.accounts.token_mint_b,
                    token_program: self.accounts.token_program,
                    system_program: self.accounts.system_program,
                    wallet: self.accounts.maker,
                }
                .invoke()?;
            }

            pinocchio_token::instructions::TransferChecked {
                from: self.accounts.taker_ata_b,
                to: self.accounts.maker_ata_b,
                authority: self.accounts.taker,
                mint: self.accounts.token_mint_b,
                amount: u64::from_le_bytes(offer.token_b_wanted_amount),
                decimals: Mint::from_account_info(self.accounts.token_mint_b)?.decimals(),
            }
            .invoke()?;
        }

        let bump_binding = [offer.bump];
        let id_binding = offer.id;
        let seed = [
            Seed::from(Offer::SEED_PREFIX),
            Seed::from(self.accounts.maker.key().as_ref()),
            Seed::from(&id_binding),
            Seed::from(&bump_binding),
        ];
        let signer_seeds = Signer::from(&seed);

        {
            let amount = {
                let vault_account = TokenAccount::from_account_info(self.accounts.vault)?;
                vault_account.amount()
            };

            pinocchio_token::instructions::TransferChecked {
                from: self.accounts.vault,
                to: self.accounts.taker_ata_a,
                authority: self.accounts.offer,
                mint: self.accounts.token_mint_a,
                amount: amount,
                decimals: Mint::from_account_info(self.accounts.token_mint_a)?.decimals(),
            }
            .invoke_signed(&[signer_seeds.clone()])?;

            pinocchio_token::instructions::CloseAccount {
                account: self.accounts.vault,
                destination: self.accounts.maker,
                authority: self.accounts.offer,
            }
            .invoke_signed(&[signer_seeds])?;
        }

        drop(data);
        // close offer account
        {
            {
                let mut data = self.accounts.offer.try_borrow_mut_data()?;
                data[0] = 0xff;
            }

            *self.accounts.maker.try_borrow_mut_lamports()? +=
                *self.accounts.offer.try_borrow_lamports()?;
            self.accounts.offer.realloc(1, true)?;
            self.accounts.offer.close()?;
        }

        Ok(())
    }
}
