//! Derived-distribution tests.
//!
//! Tests in this category verify that derived statistical quantities (sums, game
//! outcomes, counts) follow a theoretical distribution (Normal, Poisson, Markov).
//! They are distinct from frequency tests (which check raw bit/byte counts) and
//! spatial tests (which check geometric properties).
pub mod craps;
pub mod overlapping_sums;
