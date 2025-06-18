#[cfg(test)]
mod tests {

    use mollusk_svm::{
        program::loader_keys::LOADER_V3,
        result::{Check, ProgramResult},
        Mollusk,
    };
    use spl_associated_token_account::get_associated_token_address;
    use spl_token::state::AccountState;
    use transfer_tokens::{
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

    use solana_sdk::pubkey;
    use solana_sdk::pubkey::Pubkey;

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);
    pub const TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
        pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

    fn get_mollusk() -> Mollusk {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/transfer_tokens");

        mollusk.add_program(&TOKEN_PROGRAM_ID, "../../idl/token_program", &LOADER_V3);
        mollusk.add_program(
            &ASSOCIATED_TOKEN_PROGRAM_ID,
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
            spl_token::ID,
            mollusk_svm::program::create_program_account_loader_v3(&spl_token::ID),
        );

        let payer = Pubkey::new_from_array([0x02; 32]);
        let payer_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let mint = Pubkey::new_from_array([0x03; 32]);
        let mint_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data = CreateTokenInstructionData {
            token_decimals: 9,
            mint_authority: payer.to_bytes(),
            freeze_authority: payer.to_bytes(),
        };

        let ix_data_bytes = bytemuck::bytes_of(&ix_data);

        let data = [vec![0], ix_data_bytes.to_vec()].concat();

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
            spl_token::ID,
            mollusk_svm::program::create_program_account_loader_v3(&spl_token::ID),
        );

        let (associated_token_program, associated_token_program_account) = (
            ASSOCIATED_TOKEN_PROGRAM_ID,
            mollusk_svm::program::create_program_account_loader_v3(&ASSOCIATED_TOKEN_PROGRAM_ID),
        );

        let mint_authority = Pubkey::new_from_array([0x02; 32]);
        let mint_authority_account =
            AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let mint = Pubkey::new_from_array([0x03; 32]);
        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );

        let receiver = Pubkey::new_from_array([0x04; 32]);
        let receiver_account = AccountSharedData::new(0, 0, &system_program);

        let token_account_address = get_associated_token_address(&receiver, &mint);
        let token_account = AccountSharedData::new(0, 0, &system_program);

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
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

        let ix_data_bytes = bytemuck::bytes_of(&ix_data);

        let data = [vec![1], ix_data_bytes.to_vec()].concat();

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
            spl_token::ID,
            mollusk_svm::program::create_program_account_loader_v3(&spl_token::ID),
        );

        let (associated_token_program, associated_token_program_account) = (
            ASSOCIATED_TOKEN_PROGRAM_ID,
            mollusk_svm::program::create_program_account_loader_v3(&ASSOCIATED_TOKEN_PROGRAM_ID),
        );

        let from = Pubkey::new_from_array([0x02; 32]);
        let from_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

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

        let from_token_account_address = get_associated_token_address(&from, &mint);
        let mut from_token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );

        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
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

        let to_token_account_address = get_associated_token_address(&receiver, &mint);
        let to_token_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data = TransferInstructionData {
            amount: 100_000_000_000u64.to_le_bytes(),
        };

        let ix_data_bytes = bytemuck::bytes_of(&ix_data);

        let data = [vec![2], ix_data_bytes.to_vec()].concat();

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
