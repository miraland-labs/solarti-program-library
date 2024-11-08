#![allow(clippy::arithmetic_side_effects)]
#![deny(missing_docs)]

//! An Uniswap-like program for the Miraland blockchain.

pub mod constraints;
pub mod curve;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk
// version
pub use miraland_program;

miraland_program::declare_id!("Swapxy418CiVrU7RL7ZF5RuJKgncm7efhS8msyREZod");
