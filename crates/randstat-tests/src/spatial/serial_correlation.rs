//! ENT Serial Correlation test.
//!
//! `SerialCorrelationTest` is the streaming accumulator. The evaluation formula
//! lives in [`randstat_core::algorithms::serial_corr::serial_corr_result`].

use randstat_core::algorithms::serial_corr::serial_corr_result;
use randstat_core::bitstream::serial_corr::SerialCorrAccum;
use randstat_core::traits::{StreamTest, TestResult};

/// Serial correlation streaming accumulator.
#[derive(Debug, Clone, Copy, Default)]
pub struct SerialCorrelationTest {
    pub accum: SerialCorrAccum,
}

impl SerialCorrelationTest {
    pub const fn new() -> Self {
        Self {
            accum: SerialCorrAccum::new(),
        }
    }
}

impl StreamTest for SerialCorrelationTest {
    fn update(&mut self, chunk: &[u8]) {
        self.accum.update(chunk);
    }

    fn reset(&mut self) {
        self.accum.reset();
    }

    fn evaluate(&self) -> TestResult {
        serial_corr_result(self.accum.correlation())
    }
}
