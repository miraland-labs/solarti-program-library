//! Crate defining a library with a procedural macro and other
//! dependencies for building Miraland program errors

#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

extern crate self as spl_program_error;

// Make these available downstream for the macro to work without
// additional imports
pub use {
    miraland_program, num_derive, num_traits,
    spl_program_error_derive::{
        spl_program_error, DecodeError, IntoProgramError, PrintProgramError,
    },
    thiserror,
};
