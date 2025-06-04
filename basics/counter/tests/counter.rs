#[cfg(test)]
mod tests {
    use counter::{
        constants::COUNTER_SEED, instructions::CreateCounterInstructionData, state::Counter, ID,
    };
    use mollusk_svm::{
        result::{Check, ProgramResult},
        Mollusk,
    };

    use solana_sdk::{
        account::AccountSharedData,
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
    };

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    #[test]
    fn test_create_counter_data() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/counter");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let owner = Pubkey::new_from_array([0x02; 32]);
        let owner_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (counter_pubkey, bump) =
            solana_sdk::pubkey::Pubkey::find_program_address(&[COUNTER_SEED], &PROGRAM_ID);
        let counter_account = AccountSharedData::new(0, 0, &system_program);

        let counter_init_state = Counter {
            count: 100u64.to_le_bytes(),
        };

        // create counter instruction
        {
            let ix_data = CreateCounterInstructionData {
                initial_value: counter_init_state.count,
                bump,
            };

            let ix_data_bytes = bytemuck::bytes_of(&ix_data);

            let data = [vec![0], ix_data_bytes.to_vec()].concat();

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(owner, true),
                    AccountMeta::new(counter_pubkey, false),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (owner, owner_account.clone().into()),
                        (counter_pubkey, counter_account.clone().into()),
                        (system_program, system_account.clone()),
                    ],
                    &[
                        Check::success(),
                        Check::account(&counter_pubkey).owner(&PROGRAM_ID).build(),
                        Check::account(&counter_pubkey)
                            .data(bytemuck::bytes_of(&counter_init_state))
                            .build(),
                    ],
                );

            let updated_data = result.get_account(&counter_pubkey).unwrap();
            let parsed_data = bytemuck::from_bytes::<Counter>(&updated_data.data);

            assert_eq!(parsed_data.count, 100u64.to_le_bytes());
            assert!(updated_data.owner.eq(&PROGRAM_ID));

            assert!(result.program_result == ProgramResult::Success);
        }
    }

    #[test]
    fn test_increase_counter_data() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/counter");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let owner = Pubkey::new_from_array([0x02; 32]);
        let owner_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (counter_pubkey, _) =
            solana_sdk::pubkey::Pubkey::find_program_address(&[COUNTER_SEED], &PROGRAM_ID);

        let counter_init_state = Counter {
            count: 100u64.to_le_bytes(),
        };

        // increase counter instruction
        {
            let mut counter_account = AccountSharedData::new(
                mollusk.sysvars.rent.minimum_balance(Counter::LEN),
                Counter::LEN,
                &PROGRAM_ID,
            );

            counter_account.set_data_from_slice(bytemuck::bytes_of(&counter_init_state));

            let data = vec![1];

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(owner, true),
                    AccountMeta::new(counter_pubkey, true),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (owner, owner_account.into()),
                        (counter_pubkey, counter_account.into()),
                        (system_program, system_account),
                    ],
                    &[
                        Check::success(),
                        Check::account(&counter_pubkey).owner(&PROGRAM_ID).build(),
                        Check::account(&counter_pubkey)
                            .data(bytemuck::bytes_of(&Counter {
                                count: 101u64.to_le_bytes(),
                            }))
                            .build(),
                    ],
                );

            let updated_data = result.get_account(&counter_pubkey).unwrap();
            let parsed_data = bytemuck::from_bytes::<Counter>(&updated_data.data);

            assert_eq!(parsed_data.count, 101u64.to_le_bytes());

            assert!(result.program_result == ProgramResult::Success);
        }
    }

    #[test]
    fn test_decrease_counter_data() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/counter");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let owner = Pubkey::new_from_array([0x02; 32]);
        let owner_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (counter_pubkey, _) =
            solana_sdk::pubkey::Pubkey::find_program_address(&[COUNTER_SEED], &PROGRAM_ID);

        let counter_init_state = Counter {
            count: 100u64.to_le_bytes(),
        };

        // increase counter instruction
        {
            let mut counter_account = AccountSharedData::new(
                mollusk.sysvars.rent.minimum_balance(Counter::LEN),
                Counter::LEN,
                &PROGRAM_ID,
            );

            counter_account.set_data_from_slice(bytemuck::bytes_of(&counter_init_state));

            let data = vec![2];

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(owner, true),
                    AccountMeta::new(counter_pubkey, true),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (owner, owner_account.into()),
                        (counter_pubkey, counter_account.into()),
                        (system_program, system_account),
                    ],
                    &[
                        Check::success(),
                        Check::account(&counter_pubkey).owner(&PROGRAM_ID).build(),
                        Check::account(&counter_pubkey)
                            .data(bytemuck::bytes_of(&Counter {
                                count: 99u64.to_le_bytes(),
                            }))
                            .build(),
                    ],
                );

            let updated_data = result.get_account(&counter_pubkey).unwrap();
            let parsed_data = bytemuck::from_bytes::<Counter>(&updated_data.data);

            assert_eq!(parsed_data.count, 99u64.to_le_bytes());

            assert!(result.program_result == ProgramResult::Success);
        }
    }
}
