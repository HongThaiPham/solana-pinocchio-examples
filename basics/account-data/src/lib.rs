#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use processor::process_instruction;

pub mod constants;
pub mod instructions;
pub mod processor;
pub mod state;
pub mod util;
pinocchio_pubkey::declare_id!("EAUvJAw61MTaJbyV4tqFB4dEZuYHdYrtpGQ35hDsQ6Dw");

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();
