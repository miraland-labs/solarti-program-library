//! Crate defining an example program for creating Solarti token groups
//! using the Solarti Token Group interface.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
