//! Dieharder: Craps Test (stub).
//!
//! Simulates the dice game of Craps using pairs of random bytes as dice rolls.
//! Records the number of wins, losses, and the distribution of throws needed
//! to resolve each game. The expected win probability for craps is 244/495 ≈ 0.4929.
//!
//! Tests are run for the win/loss ratio (z-score) and for the throw-count
//! distribution (chi-square against theoretical Markov chain probabilities).
//!
//! **Status: Stub** — accumulates total bytes; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Craps Test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct CrapsTest {
    pub total_bytes: u64,
}

impl CrapsTest {
    pub const fn new() -> Self {
        Self { total_bytes: 0 }
    }
}

impl StreamTest for CrapsTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bytes += chunk.len() as u64;
    }
    fn reset(&mut self) {
        self.total_bytes = 0;
    }
    fn evaluate(&self) -> TestResult {
        // TODO: consume pairs of bytes as dice (each byte mod 6 + 1);
        //       simulate craps rules to record win/loss and throw counts;
        //       compare win probability via z-score and throw-count distribution
        //       via chi-square. Two p-values: min determines overall pass.
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
