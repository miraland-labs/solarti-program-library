//! binary oracle pair
#![deny(missing_docs)]

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk
// version
pub use miraland_program;

// Binary Oracle Pair id
miraland_program::declare_id!("borp1gGAKmL4HqvZSNHdVBFfnsMEZZ1ho9UCuMyya8g");
