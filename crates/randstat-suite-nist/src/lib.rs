//! `randstat-suite-nist` — Zero-alloc NIST SP800-22 test suite (all 15 tests).
//!
//! All 15 NIST SP800-22 tests are included. Tests marked ✅ have real implementations;
//! tests marked 🔧 are stubs that accumulate bytes and return `TestResult::NOT_IMPLEMENTED`.

#![no_std]

use randstat_core::traits::StreamTest;
// Frequency
use randstat_tests::frequency::arithmetic_mean::ArithmeticMeanTest;
use randstat_tests::frequency::block_frequency::BlockFrequencyTest;
use randstat_tests::frequency::chi_square::ChiSquareTest;
use randstat_tests::frequency::cusum::CusumTest;
use randstat_tests::frequency::monobit::MonobitTest;
use randstat_tests::frequency::serial_test::SerialTest;
// Runs
use randstat_tests::runs::longest_run::LongestRunTest;
use randstat_tests::runs::runs_test::RunsTest;
// Spectral
use randstat_tests::spectral::dft_fft::DftTest;
// Template
use randstat_tests::template::non_overlapping_template::NonOverlappingTemplateTest;
use randstat_tests::template::overlapping_template::OverlappingTemplateTest;
// Complexity
use randstat_tests::complexity::approx_entropy::ApproxEntropyTest;
use randstat_tests::complexity::berlekamp_massey::BerlekampMasseyTest;
use randstat_tests::complexity::maurers_universal::MaurersUniversalTest;
// Excursions
use randstat_tests::excursions::random_excursions::RandomExcursionsTest;
use randstat_tests::excursions::random_excursions_variant::RandomExcursionsVariantTest;
// Matrix
use randstat_tests::matrix::binary_matrix_rank::BinaryMatrixRankTest;

/// Zero-alloc NIST SP800-22 suite — all 15 tests.
///
/// | Test | Section | Status |
/// |---|---|---|
/// | Frequency (Monobit) | 2.1 | ✅ Real |
/// | Block Frequency     | 2.2 | ✅ Real |
/// | Runs                | 2.3 | ✅ Real |
/// | Longest Run         | 2.4 | ✅ Real |
/// | Matrix Rank         | 2.5 | 🔧 Stub |
/// | DFT/FFT Spectral    | 2.6 | 🔧 Stub |
/// | Non-overlapping Template | 2.7 | 🔧 Stub |
/// | Overlapping Template     | 2.8 | 🔧 Stub |
/// | Maurer's Universal  | 2.9 | 🔧 Stub |
/// | Berlekamp-Massey    | 2.10 | 🔧 Stub |
/// | Serial Test         | 2.11 | 🔧 Stub |
/// | Approximate Entropy | 2.12 | 🔧 Stub |
/// | CUSUM               | 2.13 | 🔧 Stub |
/// | Random Excursions   | 2.14 | 🔧 Stub |
/// | Random Excursions Variant | 2.15 | 🔧 Stub |
#[derive(Debug, Clone, Copy)]
pub struct NistSuite {
    // §2.1 Frequency
    pub monobit: MonobitTest,
    // §2.2 Block Frequency
    pub block_frequency: BlockFrequencyTest,
    // §2.3 Runs
    pub runs_test: RunsTest,
    // §2.4 Longest Run
    pub longest_run: LongestRunTest,
    // §2.5 Matrix Rank
    pub matrix_rank: BinaryMatrixRankTest,
    // §2.6 DFT/FFT
    pub dft: DftTest,
    // §2.7 Non-overlapping Template
    pub non_overlapping_template: NonOverlappingTemplateTest,
    // §2.8 Overlapping Template
    pub overlapping_template: OverlappingTemplateTest,
    // §2.9 Maurer's Universal
    pub maurers_universal: MaurersUniversalTest,
    // §2.10 Berlekamp-Massey
    pub berlekamp_massey: BerlekampMasseyTest,
    // §2.11 Serial Test
    pub serial_test: SerialTest,
    // §2.12 Approximate Entropy
    pub approx_entropy: ApproxEntropyTest,
    // §2.13 CUSUM
    pub cusum: CusumTest,
    // §2.14 Random Excursions
    pub random_excursions: RandomExcursionsTest,
    // §2.15 Random Excursions Variant
    pub random_excursions_variant: RandomExcursionsVariantTest,
    // Bonus (real implementations useful alongside NIST)
    pub chi_square: ChiSquareTest,
    pub arithmetic_mean: ArithmeticMeanTest,
}

impl NistSuite {
    /// Const zero-initialised instance — safe for `static mut` in WASM.
    pub const ZERO: Self = Self {
        monobit: MonobitTest::new(),
        block_frequency: BlockFrequencyTest::new(),
        runs_test: RunsTest::new(),
        longest_run: LongestRunTest::new(),
        matrix_rank: BinaryMatrixRankTest::new(),
        dft: DftTest::new(),
        non_overlapping_template: NonOverlappingTemplateTest::new(),
        overlapping_template: OverlappingTemplateTest::new(),
        maurers_universal: MaurersUniversalTest::new(),
        berlekamp_massey: BerlekampMasseyTest::new(),
        serial_test: SerialTest::new(),
        approx_entropy: ApproxEntropyTest::new(),
        cusum: CusumTest::new(),
        random_excursions: RandomExcursionsTest::new(),
        random_excursions_variant: RandomExcursionsVariantTest::new(),
        chi_square: ChiSquareTest::new(),
        arithmetic_mean: ArithmeticMeanTest::new(),
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.monobit.update(chunk);
        self.block_frequency.update(chunk);
        self.runs_test.update(chunk);
        self.longest_run.update(chunk);
        self.matrix_rank.update(chunk);
        self.dft.update(chunk);
        self.non_overlapping_template.update(chunk);
        self.overlapping_template.update(chunk);
        self.maurers_universal.update(chunk);
        self.berlekamp_massey.update(chunk);
        self.serial_test.update(chunk);
        self.approx_entropy.update(chunk);
        self.cusum.update(chunk);
        self.random_excursions.update(chunk);
        self.random_excursions_variant.update(chunk);
        self.chi_square.update(chunk);
        self.arithmetic_mean.update(chunk);
    }

    pub fn reset(&mut self) {
        self.monobit.reset();
        self.block_frequency.reset();
        self.runs_test.reset();
        self.longest_run.reset();
        self.matrix_rank.reset();
        self.dft.reset();
        self.non_overlapping_template.reset();
        self.overlapping_template.reset();
        self.maurers_universal.reset();
        self.berlekamp_massey.reset();
        self.serial_test.reset();
        self.approx_entropy.reset();
        self.cusum.reset();
        self.random_excursions.reset();
        self.random_excursions_variant.reset();
        self.chi_square.reset();
        self.arithmetic_mean.reset();
    }

    /// Evaluates all 15 NIST SP800-22 tests and returns a structured [`NistEvaluation`].
    pub fn evaluate(&self) -> NistEvaluation {
        use randstat_core::traits::TestStatus;

        let entries = [
            NistTestEntry {
                name: "Frequency (Monobit)",
                section: "2.1",
                result: self.monobit.evaluate(),
            },
            NistTestEntry {
                name: "Block Frequency",
                section: "2.2",
                result: self.block_frequency.evaluate(),
            },
            NistTestEntry {
                name: "Runs",
                section: "2.3",
                result: self.runs_test.evaluate(),
            },
            NistTestEntry {
                name: "Longest Run of Ones",
                section: "2.4",
                result: self.longest_run.evaluate(),
            },
            NistTestEntry {
                name: "Binary Matrix Rank",
                section: "2.5",
                result: self.matrix_rank.evaluate(),
            },
            NistTestEntry {
                name: "DFT / Spectral",
                section: "2.6",
                result: self.dft.evaluate(),
            },
            NistTestEntry {
                name: "Non-overlapping Template",
                section: "2.7",
                result: self.non_overlapping_template.evaluate(),
            },
            NistTestEntry {
                name: "Overlapping Template",
                section: "2.8",
                result: self.overlapping_template.evaluate(),
            },
            NistTestEntry {
                name: "Maurer's Universal",
                section: "2.9",
                result: self.maurers_universal.evaluate(),
            },
            NistTestEntry {
                name: "Linear Complexity (Berlekamp-Massey)",
                section: "2.10",
                result: self.berlekamp_massey.evaluate(),
            },
            NistTestEntry {
                name: "Serial Test",
                section: "2.11",
                result: self.serial_test.evaluate(),
            },
            NistTestEntry {
                name: "Approximate Entropy",
                section: "2.12",
                result: self.approx_entropy.evaluate(),
            },
            NistTestEntry {
                name: "Cumulative Sums (CUSUM)",
                section: "2.13",
                result: self.cusum.evaluate(),
            },
            NistTestEntry {
                name: "Random Excursions",
                section: "2.14",
                result: self.random_excursions.evaluate(),
            },
            NistTestEntry {
                name: "Random Excursions Variant",
                section: "2.15",
                result: self.random_excursions_variant.evaluate(),
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

        NistEvaluation {
            entries,
            total_tests: entries.len(),
            implemented_count,
            passed_count,
            failed_count,
            skipped_count,
        }
    }
}

/// Result entry for an individual NIST test.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NistTestEntry {
    pub name: &'static str,
    pub section: &'static str,
    pub result: randstat_core::traits::TestResult,
}

/// Aggregated evaluation result for the NIST SP800-22 test battery.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NistEvaluation {
    pub entries: [NistTestEntry; 15],
    pub total_tests: usize,
    pub implemented_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub skipped_count: usize,
}

impl Default for NistSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nist_suite() {
        let mut suite = NistSuite::new();
        suite.update(&[0xAA; 320]);
        assert_eq!(suite.monobit.total_bits, 2560);

        let eval = suite.evaluate();
        assert_eq!(eval.total_tests, 15);
        assert_eq!(eval.implemented_count, 4); // Monobit, Block Frequency, Runs, Longest Run

        let mut default_suite = NistSuite::default();
        default_suite.update(&[1, 2, 3]);
        default_suite.reset();
        assert_eq!(default_suite.monobit.total_bits, 0);
    }
}
