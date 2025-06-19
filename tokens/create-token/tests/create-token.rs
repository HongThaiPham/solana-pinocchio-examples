#[cfg(test)]
mod tests {

    use create_token::{instructions::CreateTokenInstructionData, ID};
    use mollusk_svm::{
        program::loader_keys::LOADER_V3,
        result::{Check, ProgramResult},
        Mollusk,
    };
    use solana_sdk::{
        account::AccountSharedData,
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
    };

    use solana_sdk::pubkey::Pubkey;

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    fn get_mollusk() -> Mollusk {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/create_token");

        mollusk.add_program(&spl_token::ID, "../../idl/token_program", &LOADER_V3);
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
}
