#[cfg(test)]
mod tests {

    use escrow::{instructions::MakeOfferInstructionData, state::Offer, ID};
    use mollusk_svm::{
        program::loader_keys::LOADER_V3,
        result::{Check, ProgramResult},
        Mollusk,
    };
    use spl_associated_token_account::get_associated_token_address;
    use spl_token::state::AccountState;

    use solana_sdk::{
        account::{AccountSharedData, WritableAccount},
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        program_option::COption,
        program_pack::Pack,
    };

    use solana_sdk::pubkey;
    use solana_sdk::pubkey::Pubkey;

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);
    pub const TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
        pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

    fn get_mollusk() -> Mollusk {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/escrow");

        mollusk.add_program(&TOKEN_PROGRAM_ID, "../../idl/token_program", &LOADER_V3);
        mollusk.add_program(
            &ASSOCIATED_TOKEN_PROGRAM_ID,
            "../../idl/associated_token_program",
            &LOADER_V3,
        );
        mollusk
    }

    #[test]
    fn test_make_offer() {
        let mollusk = get_mollusk();
        let id: u64 = 1u64;
        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let (token_program, token_program_account) = (
            spl_token::ID,
            mollusk_svm::program::create_program_account_loader_v3(&spl_token::ID),
        );

        let (associated_token_program, associated_token_program_account) = (
            ASSOCIATED_TOKEN_PROGRAM_ID,
            mollusk_svm::program::create_program_account_loader_v3(&ASSOCIATED_TOKEN_PROGRAM_ID),
        );

        let maker = Pubkey::new_from_array([0x02; 32]);
        let maker_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let mint_a = Pubkey::new_from_array([0x03; 32]);
        let mut mint_a_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000_000,
                decimals: 9,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_a_account.data_as_mut_slice(),
        )
        .unwrap();

        let mint_b = Pubkey::new_from_array([0x04; 32]);
        let mut mint_b_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000_000,
                decimals: 9,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_b_account.data_as_mut_slice(),
        )
        .unwrap();

        let maker_ata_a_address = get_associated_token_address(&maker, &mint_a);
        let mut maker_ata_a_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_a,
                owner: maker,
                amount: 100_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            maker_ata_a_account.data_as_mut_slice(),
        )
        .unwrap();

        let (offer_address, bump) = Pubkey::find_program_address(
            &[Offer::SEED_PREFIX, maker.as_ref(), &id.to_le_bytes()],
            &PROGRAM_ID,
        );

        let offer_account = AccountSharedData::new(0, 0, &system_program);

        let vault_address = get_associated_token_address(&offer_address, &mint_a);
        let vault_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data: MakeOfferInstructionData = MakeOfferInstructionData {
            id: id.to_le_bytes(),
            token_a_amount: 100_000_000_000u64.to_le_bytes(),
            token_b_wanted_amount: 50_000_000_000u64.to_le_bytes(),
            bump,
        };

        let mut data = vec![0];
        data.extend_from_slice(unsafe {
            core::slice::from_raw_parts(
                &ix_data as *const MakeOfferInstructionData as *const u8,
                core::mem::size_of::<MakeOfferInstructionData>(),
            )
        });

        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(mint_a, false),
                AccountMeta::new(mint_b, false),
                AccountMeta::new(maker_ata_a_address, false),
                AccountMeta::new(offer_address, false),
                AccountMeta::new(vault_address, false),
                AccountMeta::new_readonly(token_program, false),
                AccountMeta::new_readonly(system_program, false),
                AccountMeta::new_readonly(associated_token_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk
            .process_and_validate_instruction(
                &instruction,
                &[
                    (maker, maker_account.into()),
                    (mint_a, mint_a_account.into()),
                    (mint_b, mint_b_account.into()),
                    (maker_ata_a_address, maker_ata_a_account.into()),
                    (offer_address, offer_account.into()),
                    (vault_address, vault_account.into()),
                    (token_program, token_program_account.clone()),
                    (system_program, system_account.clone()),
                    (
                        associated_token_program,
                        associated_token_program_account.clone(),
                    ),
                ],
                &[
                    Check::success(),
                    Check::account(&offer_address).owner(&PROGRAM_ID).build(),
                    Check::account(&vault_address).owner(&token_program).build(),
                ],
            );

        let updated_offer_data = result.get_account(&offer_address).unwrap();
        let parsed_offer = Offer::load(&updated_offer_data.data).unwrap();
        assert_eq!(parsed_offer.id, id.to_le_bytes());
        assert!(Pubkey::from(parsed_offer.maker).eq(&maker));
        assert!(Pubkey::from(parsed_offer.token_mint_a).eq(&mint_a));
        assert!(Pubkey::from(parsed_offer.token_mint_b).eq(&mint_b));
        assert_eq!(
            parsed_offer.token_b_wanted_amount,
            50_000_000_000u64.to_le_bytes()
        );
        assert_eq!(parsed_offer.bump, bump);

        assert!(result.program_result == ProgramResult::Success);
    }

    #[test]
    fn test_take_offer() {
        let mollusk = get_mollusk();
        let id: u64 = 1u64;
        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let (token_program, token_program_account) = (
            spl_token::ID,
            mollusk_svm::program::create_program_account_loader_v3(&spl_token::ID),
        );

        let (associated_token_program, associated_token_program_account) = (
            ASSOCIATED_TOKEN_PROGRAM_ID,
            mollusk_svm::program::create_program_account_loader_v3(&ASSOCIATED_TOKEN_PROGRAM_ID),
        );

        let maker = Pubkey::new_from_array([0x02; 32]);
        let maker_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let taker = Pubkey::new_from_array([0x05; 32]);
        let taker_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let mint_a = Pubkey::new_from_array([0x03; 32]);
        let mut mint_a_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000_000,
                decimals: 9,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_a_account.data_as_mut_slice(),
        )
        .unwrap();

        let mint_b = Pubkey::new_from_array([0x04; 32]);
        let mut mint_b_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000_000,
                decimals: 9,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_b_account.data_as_mut_slice(),
        )
        .unwrap();

        let _maker_ata_a_address = get_associated_token_address(&maker, &mint_a);
        let mut maker_ata_a_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_a,
                owner: maker,
                amount: 100_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            maker_ata_a_account.data_as_mut_slice(),
        )
        .unwrap();

        let maker_ata_b_address = get_associated_token_address(&maker, &mint_b);
        let mut maker_ata_b_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_b,
                owner: maker,
                amount: 100_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            maker_ata_b_account.data_as_mut_slice(),
        )
        .unwrap();

        let taker_ata_a_address = get_associated_token_address(&taker, &mint_a);
        let mut taker_ata_a_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_a,
                owner: taker,
                amount: 100_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            taker_ata_a_account.data_as_mut_slice(),
        )
        .unwrap();

        let taker_ata_b_address = get_associated_token_address(&taker, &mint_b);
        let mut taker_ata_b_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_b,
                owner: taker,
                amount: 100_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            taker_ata_b_account.data_as_mut_slice(),
        )
        .unwrap();

        let (offer_address, bump) = Pubkey::find_program_address(
            &[Offer::SEED_PREFIX, maker.as_ref(), &id.to_le_bytes()],
            &PROGRAM_ID,
        );

        let mut offer_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(Offer::LEN),
            Offer::LEN,
            &PROGRAM_ID,
        );

        let offer_state = Offer {
            id: id.to_le_bytes(),
            maker: maker.to_bytes(),
            token_mint_a: mint_a.to_bytes(),
            token_mint_b: mint_b.to_bytes(),
            token_b_wanted_amount: 50_000_000_000u64.to_le_bytes(),
            bump,
        };

        offer_account.set_data_from_slice(unsafe {
            core::slice::from_raw_parts(
                &offer_state as *const Offer as *const u8,
                core::mem::size_of::<Offer>(),
            )
        });

        let vault_address = get_associated_token_address(&offer_address, &mint_a);

        let mut vault_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_a,
                owner: offer_address,
                amount: 100_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_account.data_as_mut_slice(),
        )
        .unwrap();

        let data = vec![1];

        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(taker, true),
                AccountMeta::new(mint_a, false),
                AccountMeta::new(mint_b, false),
                AccountMeta::new(taker_ata_a_address, false),
                AccountMeta::new(taker_ata_b_address, false),
                AccountMeta::new(maker, false),
                AccountMeta::new(maker_ata_b_address, false),
                AccountMeta::new(offer_address, false),
                AccountMeta::new(vault_address, false),
                AccountMeta::new_readonly(token_program, false),
                AccountMeta::new_readonly(system_program, false),
                AccountMeta::new_readonly(associated_token_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk
            .process_and_validate_instruction(
                &instruction,
                &[
                    (taker, taker_account.into()),
                    (mint_a, mint_a_account.into()),
                    (mint_b, mint_b_account.into()),
                    (taker_ata_a_address, taker_ata_a_account.into()),
                    (taker_ata_b_address, taker_ata_b_account.into()),
                    (maker, maker_account.into()),
                    (maker_ata_b_address, maker_ata_b_account.into()),
                    (offer_address, offer_account.into()),
                    (vault_address, vault_account.into()),
                    (token_program, token_program_account.clone()),
                    (system_program, system_account.clone()),
                    (
                        associated_token_program,
                        associated_token_program_account.clone(),
                    ),
                ],
                &[
                    Check::success(),
                    Check::account(&vault_address).closed().build(),
                    // Check::account(&vault_address).owner(&token_program).build(),
                ],
            );

        let updated_offer_data = result.get_account(&vault_address).unwrap();
        assert!(updated_offer_data.data.is_empty());

        assert!(result.program_result == ProgramResult::Success);
    }
}
