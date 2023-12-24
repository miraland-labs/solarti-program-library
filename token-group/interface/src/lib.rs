//! Crate defining the Solarti Token Group Interface

#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod error;
pub mod instruction;
pub mod state;

/// Namespace for all programs implementing solarti-token-group
pub const NAMESPACE: &str = "spl_token_group_interface";
