#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use processor::process_instruction;

pub mod instructions;
pub mod processor;

pinocchio_pubkey::declare_id!("4HhKzZwZVL3hjSWYaqPGHXFDkgzSwcJeH2z6aaqYtWiv");

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();
