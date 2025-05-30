#[cfg(test)]
mod tests {
    use hello_solana::ID;
    use mollusk_svm::{
        result::{Check, ProgramResult},
        Mollusk,
    };
    use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    #[test]
    fn test_hello_solana() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/hello_solana");

        let instruction = Instruction::new_with_bytes(PROGRAM_ID, &[], vec![]);

        let result: mollusk_svm::result::InstructionResult =
            mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);

        assert!(result.program_result == ProgramResult::Success);
    }
}
