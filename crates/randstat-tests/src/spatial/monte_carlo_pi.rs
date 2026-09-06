//! ENT Monte Carlo Pi estimation test.
//!
//! `MonteCarloPiTest` is the streaming accumulator. The evaluation formula lives in
//! [`randstat_core::algorithms::monte_carlo::monte_carlo_pi_result`].

use randstat_core::algorithms::monte_carlo::monte_carlo_pi_result;
use randstat_core::bitstream::mont_carlo::MonteCarloAccum;
use randstat_core::traits::{StreamTest, TestResult};

/// Monte Carlo Ãâ‚¬ estimation streaming accumulator.
#[derive(Debug, Clone, Copy, Default)]
pub struct MonteCarloPiTest {
    pub accum: MonteCarloAccum,
}

impl MonteCarloPiTest {
    pub const fn new() -> Self {
        Self {
            accum: MonteCarloAccum::new(),
        }
    }
}

impl StreamTest for MonteCarloPiTest {
    fn update(&mut self, chunk: &[u8]) {
        self.accum.update(chunk);
    }

    fn reset(&mut self) {
        self.accum.reset();
    }

    fn evaluate(&self) -> TestResult {
        monte_carlo_pi_result(self.accum.inside, self.accum.total)
    }
}
