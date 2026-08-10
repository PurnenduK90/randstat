//! Core trait definitions shared across all test implementations.
//!
//! All test structs in `randstat-tests` implement [`StreamTest`].
//! Results are returned as [`TestResult`] — a flat `#[repr(C)]` struct that
//! can be written directly into WASM linear memory by `randstat-wasm`.

/// Standardised result from any [`StreamTest`].
///
/// All fields use `f64` for consistency across platforms. The `passed` field
/// applies a two-tailed threshold at α = 0.05 by default.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestResult {
    /// The primary test statistic (e.g. S_obs for Monobit, χ² for Chi-Square).
    pub statistic: f64,
    /// Two-tailed p-value in [0.0, 1.0]. Values < α or > (1-α) indicate non-randomness.
    pub p_value: f64,
    /// Whether the sequence passes at the default α = 0.05 significance level.
    pub passed: bool,
}

/// Streaming statistical test interface.
///
/// Implementations are pure stack-allocated structs; no heap is used.
/// The test receives data in arbitrarily-sized byte chunks via [`StreamTest::update`]
/// and produces a [`TestResult`] on demand via [`StreamTest::evaluate`].
pub trait StreamTest {
    /// Feed a chunk of raw bytes into the test accumulator.
    fn update(&mut self, chunk: &[u8]);
    /// Reset all internal state to the initial (zero) condition.
    fn reset(&mut self);
    /// Compute and return the test result from all bytes seen so far.
    fn evaluate(&self) -> TestResult;
}
