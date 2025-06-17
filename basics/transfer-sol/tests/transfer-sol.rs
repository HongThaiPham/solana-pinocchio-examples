#[cfg(test)]
mod tests {

    use mollusk_svm::{
        result::{Check, ProgramResult},
        Mollusk,
    };
    use transfer_sol::{instructions::shared::TransferSolInstructionData, ID};

    use solana_sdk::{
        account::AccountSharedData,
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
    };

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    #[test]
    fn test_transfer_sol_with_program() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/transfer_sol");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let payer: Pubkey = Pubkey::new_from_array([0x02; 32]);
        let payer_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &PROGRAM_ID);

        let receipent = Pubkey::new_from_array([0x03; 32]);
        let receipent_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data = TransferSolInstructionData {
            amount: 1000000000u64,
        };

        // create counter instruction
        {
            let mut data = vec![0];
            data.extend_from_slice(unsafe {
                core::slice::from_raw_parts(
                    &ix_data as *const TransferSolInstructionData as *const u8,
                    core::mem::size_of::<TransferSolInstructionData>(),
                )
            });

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(payer, true),
                    AccountMeta::new(receipent, false),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (payer, payer_account.clone().into()),
                        (receipent, receipent_account.clone().into()),
                        (system_program, system_account.clone()),
                    ],
                    &[
                        Check::success(),
                        Check::account(&payer).lamports(0).build(),
                        Check::account(&receipent).lamports(1000000000).build(),
                    ],
                );
            assert!(result.program_result == ProgramResult::Success);
        }
    }

    #[test]
    fn test_transfer_sol_with_cpi() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/transfer_sol");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let payer = Pubkey::new_from_array([0x02; 32]);
        let payer_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let receipent = Pubkey::new_from_array([0x03; 32]);
        let receipent_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data = TransferSolInstructionData {
            amount: 1000000000u64,
        };

        // create counter instruction
        {
            let mut data = vec![1];
            data.extend_from_slice(unsafe {
                core::slice::from_raw_parts(
                    &ix_data as *const TransferSolInstructionData as *const u8,
                    core::mem::size_of::<TransferSolInstructionData>(),
                )
            });

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(payer, true),
                    AccountMeta::new(receipent, false),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (payer, payer_account.clone().into()),
                        (receipent, receipent_account.clone().into()),
                        (system_program, system_account.clone()),
                    ],
                    &[
                        Check::success(),
                        Check::account(&payer).lamports(0).build(),
                        Check::account(&receipent).lamports(1000000000).build(),
                    ],
                );
            assert!(result.program_result == ProgramResult::Success);
        }
    }
}
