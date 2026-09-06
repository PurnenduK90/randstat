//! Core trait definitions shared across all test implementations.
//!
//! All test structs in `randstat-tests` implement [`StreamTest`].
//! Results are returned as [`TestResult`] â€” a flat `#[repr(C)]` struct that
//! can be written directly into WASM linear memory by `randstat-wasm`.

/// Execution or implementation status of a [`StreamTest`].
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestStatus {
    /// Test is a stub and algorithm is not yet implemented.
    NotImplemented = 0,
    /// Test evaluated and passed within significance bounds.
    Passed = 1,
    /// Test evaluated and failed significance bounds.
    Failed = 2,
    /// Insufficient bytes or events were fed to produce a valid evaluation.
    InsufficientData = 3,
}

impl TestStatus {
    pub const fn as_str(&self) -> &'static str {
        match self {
            TestStatus::NotImplemented => "NOT IMPLEMENTED",
            TestStatus::Passed => "PASS",
            TestStatus::Failed => "FAIL",
            TestStatus::InsufficientData => "INSUFFICIENT DATA",
        }
    }
}

/// Standardised result from any [`StreamTest`].
///
/// All fields use `f64` for consistency across platforms. The `passed` field
/// applies a two-tailed threshold at Î± = 0.05 by default.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestResult {
    /// The primary test statistic (e.g. S_obs for Monobit, Ï‡Â² for Chi-Square).
    pub statistic: f64,
    /// Two-tailed p-value in [0.0, 1.0]. Values < Î± or > (1-Î±) indicate non-randomness.
    pub p_value: f64,
    /// Whether the sequence passes at the default Î± = 0.05 significance level.
    pub passed: bool,
    /// Explicit implementation and evaluation status.
    pub status: TestStatus,
}

impl TestResult {
    /// Constant stub result for tests not yet implemented.
    pub const NOT_IMPLEMENTED: Self = Self {
        statistic: f64::NAN,
        p_value: f64::NAN,
        passed: false,
        status: TestStatus::NotImplemented,
    };

    /// Constant result when insufficient data has been streamed to evaluate the test.
    pub const INSUFFICIENT_DATA: Self = Self {
        statistic: f64::NAN,
        p_value: f64::NAN,
        passed: false,
        status: TestStatus::InsufficientData,
    };

    /// Construct a passing result.
    #[inline]
    pub const fn pass(statistic: f64, p_value: f64) -> Self {
        Self {
            statistic,
            p_value,
            passed: true,
            status: TestStatus::Passed,
        }
    }

    /// Construct a failing result.
    #[inline]
    pub const fn fail(statistic: f64, p_value: f64) -> Self {
        Self {
            statistic,
            p_value,
            passed: false,
            status: TestStatus::Failed,
        }
    }

    /// Construct a result evaluated against significance level `alpha`.
    #[inline]
    pub fn from_p_value(statistic: f64, p_value: f64, alpha: f64) -> Self {
        if p_value.is_nan() {
            return Self::NOT_IMPLEMENTED;
        }
        let passed = p_value >= alpha && p_value <= (1.0 - alpha);
        Self {
            statistic,
            p_value,
            passed,
            status: if passed {
                TestStatus::Passed
            } else {
                TestStatus::Failed
            },
        }
    }
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
