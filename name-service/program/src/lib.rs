#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

// Export current sdk types for downstream users building with a different sdk
// version
pub use solana_program;

solana_program::declare_id!("NamSVdu81GY7ntoqY5Ts4ojoch1JZoetQXb8cuS8gbx");
