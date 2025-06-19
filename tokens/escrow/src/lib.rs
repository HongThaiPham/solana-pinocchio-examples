#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use processor::process_instruction;

pub mod error;
pub mod instructions;
pub mod processor;
pub mod state;
pinocchio_pubkey::declare_id!("3dCDFTjAN2w6oRZxkkBwsDC54UvoLRFV5CkM3Bg87TNi");

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();
