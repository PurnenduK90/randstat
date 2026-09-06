//! Pure stateless evaluation algorithms.
//!
//! Each function takes raw accumulator outputs (counts, sums) and returns a
//! [`TestResult`]. No streaming state, no `StreamTest` trait — just math.
//!
//! These are the canonical implementations; `randstat-tests` structs call
//! these functions from their `evaluate()` methods.

pub mod block_frequency;
pub mod chi_square;
pub mod longest_run;
pub mod monobit;
pub mod monte_carlo;
pub mod poker;
pub mod runs;
pub mod serial_corr;
pub mod shannon;
