#[cfg(test)]
mod tests {

    use mollusk_svm::{
        program::loader_keys::LOADER_V3,
        result::{Check, ProgramResult},
        Mollusk,
    };
    use pinocchio_helper::create_padded_array;
    use spl_associated_token_account::get_associated_token_address_with_program_id;
    use spl_token_2022::state::AccountState;
    use token_2022_basic::{
        instructions::{
            CreateTokenInstructionData, MintTokenInstructionData, TransferInstructionData,
        },
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

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    fn get_mollusk() -> Mollusk {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/token_2022_basic");

        mollusk.add_program(
            &spl_token_2022::ID,
            "../../idl/token_2022_program",
            &LOADER_V3,
        );
        mollusk.add_program(
            &spl_associated_token_account::ID,
            "../../idl/associated_token_program",
            &LOADER_V3,
        );
        mollusk
    }

    #[test]
    fn test_create_token() {
        let mollusk = get_mollusk();
        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let (token_program, token_program_account) = (
            spl_token_2022::ID,
            mollusk_svm::program::create_program_account_loader_v3(&spl_token_2022::ID),
        );

        let payer = Pubkey::new_from_array([0x02; 32]);
        let payer_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let mint = Pubkey::new_from_array([0x03; 32]);
        let mint_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data = CreateTokenInstructionData {
            token_name: create_padded_array(b"Solana", 32),
            token_decimals: 9,
        };

        let mut data = vec![0];
        data.extend_from_slice(unsafe {
            core::slice::from_raw_parts(
                &ix_data as *const CreateTokenInstructionData as *const u8,
                CreateTokenInstructionData::LEN,
            )
        });

        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(payer, true),
                AccountMeta::new(mint, true),
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
                    (token_program, token_program_account.clone()),
                    (system_program, system_account.clone()),
                ],
                &[
                    Check::success(),
                    Check::account(&mint).owner(&token_program).build(),
                ],
            );

        assert!(result.program_result == ProgramResult::Success);
    }

    #[test]
    fn test_mint_token() {
        let mollusk = get_mollusk();
        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let (token_program, token_program_account) = (
            spl_token_2022::ID,
            mollusk_svm::program::create_program_account_loader_v3(&spl_token_2022::ID),
        );

        let (associated_token_program, associated_token_program_account) = (
            spl_associated_token_account::ID,
            mollusk_svm::program::create_program_account_loader_v3(
                &spl_associated_token_account::ID,
            ),
        );

        let mint_authority = Pubkey::new_from_array([0x02; 32]);
        let mint_authority_account =
            AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let mint = Pubkey::new_from_array([0x03; 32]);
        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token_2022::state::Mint::LEN),
            spl_token_2022::state::Mint::LEN,
            &token_program,
        );

        let receiver = Pubkey::new_from_array([0x04; 32]);
        let receiver_account = AccountSharedData::new(0, 0, &system_program);

        let token_account_address =
            get_associated_token_address_with_program_id(&receiver, &mint, &spl_token_2022::ID);
        let token_account = AccountSharedData::new(0, 0, &system_program);

        solana_sdk::program_pack::Pack::pack(
            spl_token_2022::state::Mint {
                mint_authority: COption::Some(mint_authority),
                supply: 0,
                decimals: 9,
                is_initialized: true,
                freeze_authority: COption::Some(mint_authority),
            },
            mint_account.data_as_mut_slice(),
        )
        .unwrap();

        let ix_data = MintTokenInstructionData {
            amount: 100_000_000_000u64.to_le_bytes(),
        };

        let mut data = vec![1];
        data.extend_from_slice(unsafe {
            core::slice::from_raw_parts(
                &ix_data as *const MintTokenInstructionData as *const u8,
                MintTokenInstructionData::LEN,
            )
        });

        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(mint_authority, true),
                AccountMeta::new(mint, false),
                AccountMeta::new(receiver, false),
                AccountMeta::new(token_account_address, false),
                AccountMeta::new(associated_token_program, false),
                AccountMeta::new(token_program, false),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk
            .process_and_validate_instruction(
                &instruction,
                &[
                    (mint_authority, mint_authority_account.clone().into()),
                    (mint, mint_account.clone().into()),
                    (receiver, receiver_account.clone().into()),
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

        assert!(result.program_result == ProgramResult::Success);
    }

    #[test]
    fn test_transfer_token() {
        let mollusk = get_mollusk();
        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let (token_program, token_program_account) = (
            spl_token_2022::ID,
            mollusk_svm::program::create_program_account_loader_v3(&spl_token_2022::ID),
        );

        let (associated_token_program, associated_token_program_account) = (
            spl_associated_token_account::ID,
            mollusk_svm::program::create_program_account_loader_v3(
                &spl_associated_token_account::ID,
            ),
        );

        let from = Pubkey::new_from_array([0x02; 32]);
        let from_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let mint = Pubkey::new_from_array([0x03; 32]);
        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token_2022::state::Mint::LEN),
            spl_token_2022::state::Mint::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token_2022::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000_000,
                decimals: 9,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        )
        .unwrap();

        let receiver = Pubkey::new_from_array([0x04; 32]);
        let receiver_account = AccountSharedData::new(0, 0, &system_program);

        let from_token_account_address =
            get_associated_token_address_with_program_id(&from, &mint, &spl_token_2022::ID);

        let mut from_token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token_2022::state::Account::LEN),
            spl_token_2022::state::Account::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token_2022::state::Account {
                mint: mint,
                owner: from,
                amount: 100_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            from_token_account.data_as_mut_slice(),
        )
        .unwrap();

        let to_token_account_address =
            get_associated_token_address_with_program_id(&receiver, &mint, &spl_token_2022::ID);

        let to_token_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data = TransferInstructionData {
            amount: 100_000_000_000u64.to_le_bytes(),
        };

        let mut data = vec![2];
        data.extend_from_slice(unsafe {
            core::slice::from_raw_parts(
                &ix_data as *const TransferInstructionData as *const u8,
                TransferInstructionData::LEN,
            )
        });

        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(from, true),
                AccountMeta::new(mint, false),
                AccountMeta::new(receiver, false),
                AccountMeta::new(from_token_account_address, false),
                AccountMeta::new(to_token_account_address, false),
                AccountMeta::new(associated_token_program, false),
                AccountMeta::new(token_program, false),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk
            .process_and_validate_instruction(
                &instruction,
                &[
                    (from, from_account.clone().into()),
                    (mint, mint_account.clone().into()),
                    (receiver, receiver_account.clone().into()),
                    (
                        from_token_account_address,
                        from_token_account.clone().into(),
                    ),
                    (to_token_account_address, to_token_account.clone().into()),
                    (
                        associated_token_program,
                        associated_token_program_account.clone(),
                    ),
                    (token_program, token_program_account.clone()),
                    (system_program, system_account.clone()),
                ],
                &[
                    Check::success(),
                    Check::account(&to_token_account_address)
                        .owner(&token_program)
                        .build(),
                ],
            );

        assert!(result.program_result == ProgramResult::Success);
    }
}
