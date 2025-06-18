#![no_std]
#![allow(unexpected_cfgs)]

use pinocchio::no_allocator;
use pinocchio::nostd_panic_handler;
#[cfg(not(feature = "no-entrypoint"))]
use pinocchio::program_entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
use crate::processor::process_instruction;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod processor;
pub mod state;

pinocchio_pubkey::declare_id!("Ag8tR8rXHLwUGPCfgGUJYjcYnFnqFdJ8XfjGP5LeRpg6");

#[cfg(not(feature = "no-entrypoint"))]
program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();
