//! Streaming byte-level accumulators.
//!
//! These are the low-level building blocks used by `randstat-tests` test structs.
//! All types are `Copy`/`Clone` and contain no heap allocations.

pub mod byte_freq;
pub mod mont_carlo;
pub mod serial_corr;
pub mod sha256;
