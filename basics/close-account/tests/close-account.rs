#[cfg(test)]
mod tests {

    use close_account::{instructions::create_user::CreateUserInstructionData, state::User, ID};
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
    fn test_create_user_account() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/close_account");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let payer = Pubkey::new_from_array([0x02; 32]);
        let payer_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (user_account_pubkey, _) = solana_sdk::pubkey::Pubkey::find_program_address(
            &[User::SEED_PREFIX, payer.as_ref()],
            &PROGRAM_ID,
        );
        let user_account = AccountSharedData::new(0, 0, &system_program);

        let user_state = CreateUserInstructionData {
            name: create_padded_array(b"Leo", 64),
        };

        // create counter instruction
        {
            let mut data = vec![0];
            data.extend_from_slice(unsafe {
                core::slice::from_raw_parts(
                    &user_state as *const CreateUserInstructionData as *const u8,
                    core::mem::size_of::<CreateUserInstructionData>(),
                )
            });

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(payer, true),
                    AccountMeta::new(user_account_pubkey, false),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (payer, payer_account.clone().into()),
                        (user_account_pubkey, user_account.clone().into()),
                        (system_program, system_account.clone()),
                    ],
                    &[
                        Check::success(),
                        Check::account(&user_account_pubkey)
                            .owner(&PROGRAM_ID)
                            .build(),
                    ],
                );

            let updated_data = result.get_account(&user_account_pubkey).unwrap();
            let parsed_data = User::load(&updated_data.data).unwrap();

            assert_eq!(parsed_data.name, user_state.name);
            assert!(updated_data.owner.eq(&PROGRAM_ID));

            assert!(result.program_result == ProgramResult::Success);
        }
    }

    #[test]
    fn test_close_user_account() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/close_account");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let payer = Pubkey::new_from_array([0x02; 32]);
        let payer_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (user_account_pubkey, _) = solana_sdk::pubkey::Pubkey::find_program_address(
            &[User::SEED_PREFIX, payer.as_ref()],
            &PROGRAM_ID,
        );

        let user_state = CreateUserInstructionData {
            name: create_padded_array(b"Leo", 64),
        };

        // create counter instruction
        {
            let mut user_account = AccountSharedData::new(
                mollusk.sysvars.rent.minimum_balance(User::LEN),
                User::LEN,
                &PROGRAM_ID,
            );

            user_account.set_data_from_slice(unsafe {
                core::slice::from_raw_parts(
                    &user_state as *const CreateUserInstructionData as *const u8,
                    core::mem::size_of::<CreateUserInstructionData>(),
                )
            });

            let data = vec![1];

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(payer, true),
                    AccountMeta::new(user_account_pubkey, false),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (payer, payer_account.clone().into()),
                        (user_account_pubkey, user_account.clone().into()),
                        (system_program, system_account.clone()),
                    ],
                    &[
                        Check::success(),
                        Check::account(&user_account_pubkey).closed().build(),
                    ],
                );

            let updated_data = result.get_account(&user_account_pubkey).unwrap();
            assert!(updated_data.data.is_empty());
            assert!(result.program_result == ProgramResult::Success);
        }
    }
}
