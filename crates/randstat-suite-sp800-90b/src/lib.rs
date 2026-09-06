//! `randstat-suite-sp800-90b` — NIST SP 800-90B Min-Entropy Assessment Suite.
//!
//! Evaluates unconditioned physical noise sources (TRNG/QRNG) using the 10 NIST SP 800-90B
//! statistical min-entropy estimators across both IID and non-IID evaluation tracks.

#![no_std]

use randstat_core::traits::TestResult;

/// Zero-alloc NIST SP800-90B Min-Entropy suite (10 estimators).
#[derive(Debug, Clone, Copy, Default)]
pub struct Sp80090bSuite {
    pub total_bytes: u64,
}

impl Sp80090bSuite {
    pub const ZERO: Self = Self { total_bytes: 0 };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.total_bytes += chunk.len() as u64;
    }

    pub fn reset(&mut self) {
        self.total_bytes = 0;
    }

    pub fn evaluate(&self) -> Sp80090bEvaluation {
        let entries = [
            Sp80090bTestEntry {
                name: "Most Common Value",
                track: "Non-IID §6.3.1",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "Collision Test",
                track: "Non-IID §6.3.2",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "Markov Test",
                track: "Non-IID §6.3.3",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "Compression Test",
                track: "Non-IID §6.3.4",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "t-Tuple Test",
                track: "Non-IID §6.3.5",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "Longest Repeated Substring (LRS)",
                track: "Non-IID §6.3.6",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "Multi Most Common in Window (MMCW)",
                track: "Predictor §6.3.7",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "Lag Prediction Test",
                track: "Predictor §6.3.8",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "MultiMMC Prediction Test",
                track: "Predictor §6.3.9",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "LZ78Y Prediction Test",
                track: "Predictor §6.3.10",
                result: TestResult::NOT_IMPLEMENTED,
            },
        ];

        Sp80090bEvaluation {
            entries,
            total_tests: 10,
            implemented_count: 0,
            passed_count: 0,
            failed_count: 0,
            skipped_count: 10,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sp80090bTestEntry {
    pub name: &'static str,
    pub track: &'static str,
    pub result: TestResult,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sp80090bEvaluation {
    pub entries: [Sp80090bTestEntry; 10],
    pub total_tests: usize,
    pub implemented_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub skipped_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sp80090b_suite() {
        let mut suite = Sp80090bSuite::new();
        suite.update(&[1, 2, 3, 4]);
        assert_eq!(suite.total_bytes, 4);
        let eval = suite.evaluate();
        assert_eq!(eval.total_tests, 10);
        assert_eq!(eval.skipped_count, 10);
        suite.reset();
        assert_eq!(suite.total_bytes, 0);
    }
}
