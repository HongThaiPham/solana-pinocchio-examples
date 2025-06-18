#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use processor::process_instruction;

pub mod instructions;
pub mod processor;

pinocchio_pubkey::declare_id!("48njBs5KjS8fQp4LF2QCo2V7G5vjwnmFGiAYDKkYzkZa");

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();
