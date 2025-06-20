#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{no_allocator, program_entrypoint};
use processor::process_instruction;

pub mod instructions;
pub mod processor;

pinocchio_pubkey::declare_id!("DfKryFwKhmTHR7qjaUxKDsQR8BT5jdRJJ6tf4CP82eaC");

program_entrypoint!(process_instruction);
no_allocator!();
// nostd_panic_handler!();
