#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use processor::process_instruction;

pub mod instructions;
pub mod processor;
pub mod state;

pinocchio_pubkey::declare_id!("5JTY1h4jWJ2JfoJMxNtr6J9T1PkT2aP5mVTBm72UAhzf");

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();
