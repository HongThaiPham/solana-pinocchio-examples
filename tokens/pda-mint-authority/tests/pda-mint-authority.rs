#[cfg(test)]
mod tests {

    use mollusk_svm::{
        program::loader_keys::LOADER_V3,
        result::{Check, ProgramResult},
        Mollusk,
    };
    use pda_mint_authority::{
        instructions::{
            CreateTokenInstructionData, InitMintAuthorityInstructionData, MintTokenInstructionData,
        },
        state::MintAuthority,
        ID,
    };
    use solana_sdk::{
        account::{AccountSharedData, WritableAccount},
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        program_option::COption,
        program_pack::Pack,
    };

    use solana_sdk::pubkey::Pubkey;
    use spl_associated_token_account::get_associated_token_address;

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    fn get_mollusk() -> Mollusk {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/pda_mint_authority");

        mollusk.add_program(&spl_token::ID, "../../idl/token_program", &LOADER_V3);
        mollusk.add_program(
            &spl_associated_token_account::ID,
            "../../idl/associated_token_program",
            &LOADER_V3,
        );
        mollusk
    }

    #[test]
    fn test_init_mint_authority() {
        let mollusk = get_mollusk();

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let payer = Pubkey::new_from_array([0x02; 32]);
        let payer_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (mint_authority_pubkey, bump) = solana_sdk::pubkey::Pubkey::find_program_address(
            &[MintAuthority::SEED_PREFIX],
            &PROGRAM_ID,
        );

        let mint_authority_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data = InitMintAuthorityInstructionData { bump };

        let mut data = vec![0];
        data.extend_from_slice(unsafe {
            core::slice::from_raw_parts(
                &ix_data as *const InitMintAuthorityInstructionData as *const u8,
                core::mem::size_of::<InitMintAuthorityInstructionData>(),
            )
        });

        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(payer, true),
                AccountMeta::new(mint_authority_pubkey, true),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk
            .process_and_validate_instruction(
                &instruction,
                &[
                    (payer, payer_account.clone().into()),
                    (mint_authority_pubkey, mint_authority_account.clone().into()),
                    (system_program, system_account.clone()),
                ],
                &[
                    Check::success(),
                    Check::account(&mint_authority_pubkey)
                        .owner(&PROGRAM_ID)
                        .build(),
                ],
            );

        let mint_authority_data = result.get_account(&mint_authority_pubkey).unwrap();
        let mint_authority_state = MintAuthority::load(&mint_authority_data.data).unwrap();
        assert_eq!(mint_authority_state.bump, bump);

        assert!(result.program_result == ProgramResult::Success);
    }

    #[test]
    fn test_create_token() {
        let mollusk = get_mollusk();
        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let (token_program, token_program_account) = (
            spl_token::ID,
            mollusk_svm::program::create_program_account_loader_v3(&spl_token::ID),
        );

        let payer = Pubkey::new_from_array([0x02; 32]);
        let payer_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (mint_authority_pubkey, bump) = solana_sdk::pubkey::Pubkey::find_program_address(
            &[MintAuthority::SEED_PREFIX],
            &PROGRAM_ID,
        );
        let mint_authority_state = MintAuthority { bump };

        let mut mint_authority_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(MintAuthority::LEN),
            MintAuthority::LEN,
            &PROGRAM_ID,
        );

        mint_authority_account.set_data_from_slice(unsafe {
            core::slice::from_raw_parts(
                &mint_authority_state as *const MintAuthority as *const u8,
                core::mem::size_of::<MintAuthority>(),
            )
        });

        let mint = Pubkey::new_from_array([0x03; 32]);
        let mint_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data = CreateTokenInstructionData {
            token_decimals: 9,
            mint_authority: mint_authority_pubkey.to_bytes(),
            freeze_authority: payer.to_bytes(),
        };

        let mut data = vec![1];
        data.extend_from_slice(unsafe {
            core::slice::from_raw_parts(
                &ix_data as *const CreateTokenInstructionData as *const u8,
                core::mem::size_of::<CreateTokenInstructionData>(),
            )
        });

        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(payer, true),
                AccountMeta::new(mint, true),
                AccountMeta::new(mint_authority_pubkey, false),
                AccountMeta::new(token_program, false),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk
            .process_and_validate_instruction(
                &instruction,
                &[
                    (payer, payer_account.clone().into()),
                    (mint, mint_account.clone().into()),
                    (mint_authority_pubkey, mint_authority_account.clone().into()),
                    (token_program, token_program_account.clone()),
                    (system_program, system_account.clone()),
                ],
                &[
                    Check::success(),
                    Check::account(&mint).owner(&token_program).build(),
                ],
            );

        let mint_data = result.get_account(&mint).unwrap();
        let mint_state = spl_token::state::Mint::unpack(&mint_data.data).unwrap();
        assert_eq!(
            mint_state.mint_authority,
            COption::Some(mint_authority_pubkey)
        );
        assert_eq!(mint_state.supply, 0);
        assert_eq!(mint_state.decimals, 9);
        assert!(mint_state.is_initialized);

        assert!(result.program_result == ProgramResult::Success);
    }

    #[test]
    fn test_mint_token() {
        let mollusk = get_mollusk();
        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let (token_program, token_program_account) = (
            spl_token::ID,
            mollusk_svm::program::create_program_account_loader_v3(&spl_token::ID),
        );

        let (associated_token_program, associated_token_program_account) = (
            spl_associated_token_account::ID,
            mollusk_svm::program::create_program_account_loader_v3(
                &spl_associated_token_account::ID,
            ),
        );

        let payer = Pubkey::new_from_array([0x02; 32]);
        let payer_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (mint_authority_pubkey, bump) = solana_sdk::pubkey::Pubkey::find_program_address(
            &[MintAuthority::SEED_PREFIX],
            &PROGRAM_ID,
        );
        let mint_authority_state = MintAuthority { bump };

        let mut mint_authority_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(MintAuthority::LEN),
            MintAuthority::LEN,
            &PROGRAM_ID,
        );

        mint_authority_account.set_data_from_slice(unsafe {
            core::slice::from_raw_parts(
                &mint_authority_state as *const MintAuthority as *const u8,
                core::mem::size_of::<MintAuthority>(),
            )
        });

        let mint = Pubkey::new_from_array([0x03; 32]);

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::Some(mint_authority_pubkey),
                supply: 0,
                decimals: 9,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        )
        .unwrap();

        let token_account_address = get_associated_token_address(&payer, &mint);
        let token_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data = MintTokenInstructionData {
            amount: 100_000_000_000u64.to_le_bytes(),
        };

        let mut data = vec![2];
        data.extend_from_slice(unsafe {
            core::slice::from_raw_parts(
                &ix_data as *const MintTokenInstructionData as *const u8,
                core::mem::size_of::<MintTokenInstructionData>(),
            )
        });

        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(payer, true),
                AccountMeta::new(mint, false),
                AccountMeta::new(payer, false),
                AccountMeta::new(mint_authority_pubkey, false),
                AccountMeta::new(token_account_address, false),
                AccountMeta::new_readonly(associated_token_program, false),
                AccountMeta::new_readonly(token_program, false),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk
            .process_and_validate_instruction(
                &instruction,
                &[
                    (payer, payer_account.clone().into()),
                    (mint, mint_account.clone().into()),
                    (payer, payer_account.clone().into()),
                    (mint_authority_pubkey, mint_authority_account.clone().into()),
                    (token_account_address, token_account.clone().into()),
                    (
                        associated_token_program,
                        associated_token_program_account.clone(),
                    ),
                    (token_program, token_program_account.clone()),
                    (system_program, system_account.clone()),
                ],
                &[
                    Check::success(),
                    Check::account(&token_account_address)
                        .owner(&token_program)
                        .build(),
                ],
            );

        let mint_data = result.get_account(&mint).unwrap();
        let mint_state = spl_token::state::Mint::unpack(&mint_data.data).unwrap();

        assert_eq!(mint_state.supply, 100_000_000_000);

        let token_account_data = result.get_account(&token_account_address).unwrap();
        let token_account_state =
            spl_token::state::Account::unpack(&token_account_data.data).unwrap();
        assert_eq!(token_account_state.amount, 100_000_000_000);

        assert!(result.program_result == ProgramResult::Success);
    }
}
