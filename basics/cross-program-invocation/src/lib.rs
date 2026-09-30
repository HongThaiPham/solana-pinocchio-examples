#![no_std]
use pinocchio::program_entrypoint;

// use pinocchio::{no_allocator, nostd_panic_handler};

use crate::processor::process_instruction;

pub mod instructions;
pub mod processor;

pinocchio_Address::declare_id!("HgeJhsevaynVUxZdwD5RJxuubfMagRmfkj9dHDHidwVY");

program_entrypoint!(process_instruction);
// no_allocator!();
// nostd_panic_handler!();
