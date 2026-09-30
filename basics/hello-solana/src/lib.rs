#![allow(unexpected_cfgs)]
#![no_std]

use pinocchio::{
    account_info::AccountView, no_allocator, nostd_panic_handler, program_entrypoint,
    Address, ProgramResult,
};
use pinocchio_log::log;

pinocchio_Address::declare_id!("FpFC3vEsjXKTrLweeD9PaG4HpTqMNJNoMvSVcZVJ8JCT");

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();

#[inline(always)]
fn process_instruction(
    program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    log!("Hello, Solana!");
    log!("Our program's Program ID: {}", program_id);
    Ok(())
}
