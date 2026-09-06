//! `randstat-suite-quick` — Quick Randomness Screening & Health Diagnostic Suite.
//!
//! Evaluates streaming byte sequences across all 5 core mathematical dimensions of randomness:
//! 1. Information Density (Shannon Entropy, Min-Entropy H_inf, Optimum Compression)
//! 2. Frequency & Uniformity (Byte Chi-Square df=255, Arithmetic Mean)
//! 3. Bit-Level & Runs (Monobit Frequency, Runs Test)
//! 4. Pattern & Clustering (Poker Test 4-bit nibbles)
//! 5. Dependency & Geometry (Serial Correlation Lag-1, Monte Carlo Pi)

#![no_std]

use randstat_core::algorithms::shannon::shannon_score;
use randstat_core::stats::ent_result::EntResult;
use randstat_core::stats::validate::GuardrailEvaluation;
use randstat_core::traits::{StreamTest, TestResult, TestStatus};
use randstat_suite_ent::EntSuite;
use randstat_tests::frequency::monobit::MonobitTest;
use randstat_tests::frequency::poker::PokerTest;
use randstat_tests::runs::runs_test::RunsTest;

/// Zero-alloc Quick Diagnostic Streaming Battery.
#[derive(Debug, Clone, Copy)]
pub struct QuickSuite {
    pub ent: EntSuite,
    pub monobit: MonobitTest,
    pub runs: RunsTest,
    pub poker: PokerTest,
    pub total_bytes: u64,
}

impl QuickSuite {
    pub const ZERO: Self = Self {
        ent: EntSuite::ZERO,
        monobit: MonobitTest::new(),
        runs: RunsTest::new(),
        poker: PokerTest::new(),
        total_bytes: 0,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.ent.update(chunk);
        self.monobit.update(chunk);
        self.runs.update(chunk);
        self.poker.update(chunk);
        self.total_bytes += chunk.len() as u64;
    }

    pub fn reset(&mut self) {
        self.ent.reset();
        self.monobit.reset();
        self.runs.reset();
        self.poker.reset();
        self.total_bytes = 0;
    }

    /// Evaluates all 10 quick metrics and returns structured evaluation.
    pub fn evaluate(&self) -> QuickEvaluation {
        let ent_res = self.ent.finalize().unwrap_or_default();

        // Calculate Min-Entropy H_inf = -log2(p_max)
        let byte_counts = &self.ent.shannon.tracker.byte_counts;
        let mut max_count = 0u64;
        for &count in byte_counts {
            if count > max_count {
                max_count = count;
            }
        }
        let min_entropy = if self.total_bytes > 0 {
            let p_max = (max_count as f64) / (self.total_bytes as f64);
            if p_max > 0.0 {
                -libm::log2(p_max)
            } else {
                8.0
            }
        } else {
            0.0
        };

        let min_ent_result = TestResult {
            statistic: min_entropy,
            p_value: (min_entropy / 8.0).min(1.0),
            passed: min_entropy >= 7.0,
            status: if self.total_bytes < 256 {
                TestStatus::InsufficientData
            } else if min_entropy >= 7.0 {
                TestStatus::Passed
            } else {
                TestStatus::Failed
            },
        };

        let mean_diff = libm::fabs(ent_res.mean - 127.5);
        let sc_abs = libm::fabs(ent_res.serial_correlation);
        let pi_diff = libm::fabs(ent_res.monte_carlo_pi - core::f64::consts::PI);

        let entries = [
            QuickTestEntry {
                name: "Shannon Entropy",
                dimension: "Information",
                result: shannon_score(byte_counts, self.total_bytes),
            },
            QuickTestEntry {
                name: "Min-Entropy (H∞)",
                dimension: "Information",
                result: min_ent_result,
            },
            QuickTestEntry {
                name: "Optimum Compression",
                dimension: "Information",
                result: TestResult {
                    statistic: ent_res.compression_percent,
                    p_value: (100.0 - ent_res.compression_percent) / 100.0,
                    passed: ent_res.compression_percent < 5.0,
                    status: if self.total_bytes < 256 {
                        TestStatus::InsufficientData
                    } else if ent_res.compression_percent < 5.0 {
                        TestStatus::Passed
                    } else {
                        TestStatus::Failed
                    },
                },
            },
            QuickTestEntry {
                name: "Byte Uniformity (χ²)",
                dimension: "Frequency",
                result: TestResult {
                    statistic: ent_res.chi_square,
                    p_value: ent_res.entropy_bits_per_byte / 8.0,
                    passed: ent_res.chi_square >= 200.0 && ent_res.chi_square <= 310.0,
                    status: if self.total_bytes < 256 {
                        TestStatus::InsufficientData
                    } else if ent_res.chi_square >= 200.0 && ent_res.chi_square <= 310.0 {
                        TestStatus::Passed
                    } else {
                        TestStatus::Failed
                    },
                },
            },
            QuickTestEntry {
                name: "Arithmetic Mean",
                dimension: "Frequency",
                result: TestResult {
                    statistic: ent_res.mean,
                    p_value: 1.0 - (mean_diff / 127.5),
                    passed: mean_diff < 5.0,
                    status: if self.total_bytes < 256 {
                        TestStatus::InsufficientData
                    } else if mean_diff < 5.0 {
                        TestStatus::Passed
                    } else {
                        TestStatus::Failed
                    },
                },
            },
            QuickTestEntry {
                name: "Monobit Frequency",
                dimension: "Bit Distribution",
                result: self.monobit.evaluate(),
            },
            QuickTestEntry {
                name: "Runs Structure",
                dimension: "Bit Transitions",
                result: self.runs.evaluate(),
            },
            QuickTestEntry {
                name: "Poker Test (4-bit)",
                dimension: "Patterns",
                result: self.poker.evaluate(),
            },
            QuickTestEntry {
                name: "Serial Correlation (Lag-1)",
                dimension: "Dependence",
                result: TestResult {
                    statistic: ent_res.serial_correlation,
                    p_value: 1.0 - sc_abs,
                    passed: sc_abs < 0.05,
                    status: if self.total_bytes < 256 {
                        TestStatus::InsufficientData
                    } else if sc_abs < 0.05 {
                        TestStatus::Passed
                    } else {
                        TestStatus::Failed
                    },
                },
            },
            QuickTestEntry {
                name: "Monte Carlo Pi",
                dimension: "Geometry",
                result: TestResult {
                    statistic: ent_res.monte_carlo_pi,
                    p_value: 1.0 - (pi_diff / core::f64::consts::PI),
                    passed: pi_diff < 0.1,
                    status: if self.total_bytes < 256 {
                        TestStatus::InsufficientData
                    } else if pi_diff < 0.1 {
                        TestStatus::Passed
                    } else {
                        TestStatus::Failed
                    },
                },
            },
        ];

        let mut implemented_count = 0;
        let mut passed_count = 0;
        let mut failed_count = 0;
        let mut skipped_count = 0;

        for entry in &entries {
            match entry.result.status {
                TestStatus::Passed => {
                    implemented_count += 1;
                    passed_count += 1;
                }
                TestStatus::Failed => {
                    implemented_count += 1;
                    failed_count += 1;
                }
                TestStatus::NotImplemented | TestStatus::InsufficientData => {
                    skipped_count += 1;
                }
            }
        }

        QuickEvaluation {
            entries,
            total_tests: entries.len(),
            implemented_count,
            passed_count,
            failed_count,
            skipped_count,
            ent_result: ent_res,
        }
    }

    pub fn validate(&self, alpha: f64) -> Option<GuardrailEvaluation> {
        self.ent.evaluate(alpha)
    }
}

impl Default for QuickSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuickTestEntry {
    pub name: &'static str,
    pub dimension: &'static str,
    pub result: TestResult,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuickEvaluation {
    pub entries: [QuickTestEntry; 10],
    pub total_tests: usize,
    pub implemented_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub skipped_count: usize,
    pub ent_result: EntResult,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_suite() {
        let mut suite = QuickSuite::new();
        let sample = [0xAA; 512];
        suite.update(&sample);
        assert_eq!(suite.total_bytes, 512);

        let eval = suite.evaluate();
        assert_eq!(eval.total_tests, 10);
        assert_eq!(eval.implemented_count, 10);
        assert_eq!(eval.skipped_count, 0);

        suite.reset();
        assert_eq!(suite.total_bytes, 0);
    }
}
