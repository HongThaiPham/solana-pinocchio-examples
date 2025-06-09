#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use processor::process_instruction;

pub mod constants;
pub mod instructions;
pub mod processor;

pinocchio_pubkey::declare_id!("Es5r1Qu1cAFFDFVKh7Usvd2gjQbhoQ58Yo1rkX8PaUr1");

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();
