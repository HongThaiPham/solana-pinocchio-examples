#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use processor::process_instruction;

pub mod constants;
pub mod instructions;
pub mod processor;

pinocchio_pubkey::declare_id!("21cBfH1aaKVeq86icK5pq51FM3ak2QQJ1TWoMLcTTK34");

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();
