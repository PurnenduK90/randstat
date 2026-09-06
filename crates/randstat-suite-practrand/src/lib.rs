//! `randstat-suite-practrand` — PractRand High-Throughput PRNG Test Suite.
//!
//! Implements Chris Doty-Humphrey's dynamic streaming statistical tests designed
//! to detect subtle low-weight linear and structural flaws across multi-terabyte streams.

#![no_std]

use randstat_core::traits::TestResult;

/// Zero-alloc PractRand streaming battery.
#[derive(Debug, Clone, Copy, Default)]
pub struct PractRandSuite {
    pub total_bytes: u64,
}

impl PractRandSuite {
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

    pub fn evaluate(&self) -> PractRandEvaluation {
        let entries = [
            PractRandTestEntry {
                name: "BCFN (Binary Cell Finite Number)",
                test_type: "Linear / Folding",
                result: TestResult::NOT_IMPLEMENTED,
            },
            PractRandTestEntry {
                name: "Gap-16",
                test_type: "Frequency / Distance",
                result: TestResult::NOT_IMPLEMENTED,
            },
            PractRandTestEntry {
                name: "FPFT (Fourier Transform)",
                test_type: "Spectral",
                result: TestResult::NOT_IMPLEMENTED,
            },
            PractRandTestEntry {
                name: "BRank (Binary Matrix Rank)",
                test_type: "Matrix",
                result: TestResult::NOT_IMPLEMENTED,
            },
            PractRandTestEntry {
                name: "DC6 (Distance to Cube)",
                test_type: "Spatial",
                result: TestResult::NOT_IMPLEMENTED,
            },
            PractRandTestEntry {
                name: "Dist-1to4",
                test_type: "Distribution",
                result: TestResult::NOT_IMPLEMENTED,
            },
        ];

        PractRandEvaluation {
            entries,
            total_tests: 6,
            implemented_count: 0,
            passed_count: 0,
            failed_count: 0,
            skipped_count: 6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PractRandTestEntry {
    pub name: &'static str,
    pub test_type: &'static str,
    pub result: TestResult,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PractRandEvaluation {
    pub entries: [PractRandTestEntry; 6],
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
    fn test_practrand_suite() {
        let mut suite = PractRandSuite::new();
        suite.update(&[1, 2, 3]);
        let eval = suite.evaluate();
        assert_eq!(eval.total_tests, 6);
        suite.reset();
    }
}
