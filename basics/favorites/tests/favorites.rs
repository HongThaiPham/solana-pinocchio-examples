#[cfg(test)]
mod tests {

    use favorites::{
        constants::FAVORITES_SEED, instructions::CreatePdaInstructionData, state::Favorites, ID,
    };
    use mollusk_svm::{
        result::{Check, ProgramResult},
        Mollusk,
    };

    use pinocchio_helper::create_padded_array;
    use solana_sdk::{
        account::AccountSharedData,
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
    };

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    #[test]
    fn test_create_pda() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/favorites");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let user = Pubkey::new_from_array([0x02; 32]);
        let user_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (favorites_pubkey, bump) = solana_sdk::pubkey::Pubkey::find_program_address(
            &[FAVORITES_SEED, user.as_ref()],
            &PROGRAM_ID,
        );
        let favorites_account = AccountSharedData::new(0, 0, &system_program);

        let favorites_state = Favorites {
            number: 1u64.to_le_bytes(),
            color: create_padded_array(b"#FFFFFF", 50),
            hobbies: [
                create_padded_array(b"Reading", 50),
                create_padded_array(b"Gaming", 50),
                create_padded_array(b"Cooking", 50),
                create_padded_array(b"Traveling", 50),
                create_padded_array(b"Swimming", 50),
            ],
            bump,
        };

        {
            let ix_data = CreatePdaInstructionData {
                number: favorites_state.number,
                color: favorites_state.color,
                hobbies: favorites_state.hobbies,
                bump: favorites_state.bump,
            };

            let ix_data_bytes = bytemuck::bytes_of(&ix_data);

            let data = [vec![0], ix_data_bytes.to_vec()].concat();

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(user, true),
                    AccountMeta::new(favorites_pubkey, true),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (user, user_account.clone().into()),
                        (favorites_pubkey, favorites_account.clone().into()),
                        (system_program, system_account.clone()),
                    ],
                    &[
                        Check::success(),
                        Check::account(&favorites_pubkey).owner(&PROGRAM_ID).build(),
                        Check::account(&favorites_pubkey)
                            .data(bytemuck::bytes_of(&favorites_state))
                            .build(),
                    ],
                );

            let updated_data = result.get_account(&favorites_pubkey).unwrap();
            let parsed_data = bytemuck::from_bytes::<Favorites>(&updated_data.data);

            assert_eq!(parsed_data.number, favorites_state.number);
            assert_eq!(parsed_data.color, favorites_state.color);
            assert_eq!(parsed_data.hobbies, favorites_state.hobbies);
            assert_eq!(parsed_data.bump, favorites_state.bump);
            assert!(updated_data.owner.eq(&PROGRAM_ID));

            assert!(result.program_result == ProgramResult::Success);
        }
    }

    #[test]
    fn test_get_pda() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/favorites");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let user = Pubkey::new_from_array([0x02; 32]);
        let user_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (favorites_pubkey, bump) = solana_sdk::pubkey::Pubkey::find_program_address(
            &[FAVORITES_SEED, user.as_ref()],
            &PROGRAM_ID,
        );

        let favorites_state = Favorites {
            number: 1u64.to_le_bytes(),
            color: create_padded_array(b"#FFFFFF", 50),
            hobbies: [
                create_padded_array(b"Reading", 50),
                create_padded_array(b"Gaming", 50),
                create_padded_array(b"Cooking", 50),
                create_padded_array(b"Traveling", 50),
                create_padded_array(b"Swimming", 50),
            ],
            bump,
        };
        {
            let mut favorites_account = AccountSharedData::new(
                mollusk.sysvars.rent.minimum_balance(Favorites::LEN),
                Favorites::LEN,
                &PROGRAM_ID,
            );

            favorites_account.set_data_from_slice(bytemuck::bytes_of(&favorites_state));

            let data = vec![1];

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(user, true),
                    AccountMeta::new(favorites_pubkey, true),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (user, user_account.into()),
                        (favorites_pubkey, favorites_account.into()),
                        (system_program, system_account),
                    ],
                    &[
                        Check::success(),
                        Check::account(&favorites_pubkey).owner(&PROGRAM_ID).build(),
                        Check::account(&favorites_pubkey)
                            .data(bytemuck::bytes_of(&favorites_state))
                            .build(),
                    ],
                );

            let updated_data = result.get_account(&favorites_pubkey).unwrap();
            let parsed_data = bytemuck::from_bytes::<Favorites>(&updated_data.data);

            assert_eq!(parsed_data.number, favorites_state.number);
            assert_eq!(parsed_data.color, favorites_state.color);
            assert_eq!(parsed_data.hobbies, favorites_state.hobbies);
            assert_eq!(parsed_data.bump, favorites_state.bump);

            assert!(result.program_result == ProgramResult::Success);
        }
    }
}
