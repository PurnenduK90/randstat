//! Dieharder: Parking Lot Test (stub).
//!
//! Attempts to "park" random unit-radius circles in a 100Ãƒâ€”100 square by placing
//! circle centres at uniform random (x, y) positions. A circle is parked only if
//! it does not overlap any previously parked circle. After 12,000 attempts, the
//! number of parked circles should be approximately normally distributed
//! (ÃŽÂ¼ Ã¢â€°Ë† 3523, ÃÆ’ Ã¢â€°Ë† 22) for a truly random source.
//!
//! **Status: Stub** Ã¢â‚¬â€ accumulates total bytes; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Parking Lot Test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct ParkingLotTest {
    pub total_bytes: u64,
}

impl ParkingLotTest {
    pub const fn new() -> Self {
        Self { total_bytes: 0 }
    }
}

impl StreamTest for ParkingLotTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bytes += chunk.len() as u64;
    }
    fn reset(&mut self) {
        self.total_bytes = 0;
    }
    fn evaluate(&self) -> TestResult {
        // TODO: decode pairs of f32 as (x, y) Ã¢Ë†Ë† [0, 100)Ã‚Â²; for each attempt,
        //       check distance to all parked circles (distance > 2 to park);
        //       count parked circles after 12000 attempts; apply z-score against
        //       Normal(3523, 22Ã‚Â²).
        TestResult::NOT_IMPLEMENTED
    }
}
