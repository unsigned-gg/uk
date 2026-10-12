//! uk (Unsigned Kernel & Microkernel Primitives)
//!
//! Pure-Rust microkernel domain abstractions, zero-cost systems interfaces, and bare-metal sympathy.
//!
//! # Architecture
//! Anchored in the systems principles of **Bryan Cantrill & Ralf Jung**.


pub use uk_domain::*;

/// Initializes the `uk` systems engine.
pub fn init() -> anyhow::Result<()> {
    Ok(())
}
