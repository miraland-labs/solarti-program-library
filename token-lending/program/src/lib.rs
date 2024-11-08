#![allow(clippy::arithmetic_side_effects)]
#![deny(missing_docs)]

//! A lending program for the Miraland blockchain.

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod math;
pub mod processor;
pub mod pyth;
pub mod state;

// Export current sdk types for downstream users building with a different sdk version
pub use miraland_program;

miraland_program::declare_id!("Lendom2HkAKTe8bLUMxDxpBd8kCYUbZjECGtx9RFV7c");
