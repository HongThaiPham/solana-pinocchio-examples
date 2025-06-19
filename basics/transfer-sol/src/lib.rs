#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use processor::process_instruction;

pub mod instructions;
pub mod processor;

pinocchio_pubkey::declare_id!("QBDA4wAjJpX1rmpW7g6eSdize5Dq4mHbnRxkfNQCWya");

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();
