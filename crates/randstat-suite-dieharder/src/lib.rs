//! `randstat-suite-dieharder` — Dieharder test suite.
//!
//! Aggregates the 12 Dieharder-specific tests.
//! Tests are imported from their **functional category** inside `randstat-tests`.

#![no_std]

use randstat_core::traits::{StreamTest, TestResult};

// Spatial tests
use randstat_tests::spatial::birthday_spacings::BirthdaySpacingsTest;
use randstat_tests::spatial::minimum_distance_2d::MinimumDistance2DTest;
use randstat_tests::spatial::parking_lot::ParkingLotTest;
use randstat_tests::spatial::spheres_3d::Spheres3DTest;

// Runs tests
use randstat_tests::runs::operm5::Operm5Test;
use randstat_tests::runs::runs_test::RunsTest;
use randstat_tests::runs::runs_up_down::RunsUpDownTest;

// Template / occupancy tests
use randstat_tests::template::dna::DnaTest;
use randstat_tests::template::oqso::OqsoTest;

// Frequency tests
use randstat_tests::frequency::chi_square::ChiSquareTest;
use randstat_tests::frequency::count_ones_stream::CountOnesStreamTest;
use randstat_tests::frequency::monobit::MonobitTest;

// Complexity tests
use randstat_tests::complexity::squeeze::SqueezeTest;

// Distribution tests
use randstat_tests::distribution::craps::CrapsTest;
use randstat_tests::distribution::overlapping_sums::OverlappingSumsTest;

/// Zero-alloc Dieharder-specific suite.
#[derive(Debug, Clone, Copy)]
pub struct DieharderSuite {
    // Spatial
    pub birthday_spacings: BirthdaySpacingsTest,
    pub parking_lot: ParkingLotTest,
    pub minimum_distance_2d: MinimumDistance2DTest,
    pub spheres_3d: Spheres3DTest,
    // Runs
    pub runs_up_down: RunsUpDownTest,
    pub operm5: Operm5Test,
    pub runs_test: RunsTest,
    // Template / occupancy
    pub oqso: OqsoTest,
    pub dna: DnaTest,
    // Frequency
    pub count_ones_stream: CountOnesStreamTest,
    pub monobit: MonobitTest,
    pub chi_square: ChiSquareTest,
    // Complexity
    pub squeeze: SqueezeTest,
    // Distribution
    pub overlapping_sums: OverlappingSumsTest,
    pub craps: CrapsTest,
}

impl DieharderSuite {
    pub const ZERO: Self = Self {
        birthday_spacings: BirthdaySpacingsTest::new(),
        parking_lot: ParkingLotTest::new(),
        minimum_distance_2d: MinimumDistance2DTest::new(),
        spheres_3d: Spheres3DTest::new(),
        runs_up_down: RunsUpDownTest::new(),
        operm5: Operm5Test::new(),
        runs_test: RunsTest::new(),
        oqso: OqsoTest::new(),
        dna: DnaTest::new(),
        count_ones_stream: CountOnesStreamTest::new(),
        monobit: MonobitTest::new(),
        chi_square: ChiSquareTest::new(),
        squeeze: SqueezeTest::new(),
        overlapping_sums: OverlappingSumsTest::new(),
        craps: CrapsTest::new(),
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.birthday_spacings.update(chunk);
        self.parking_lot.update(chunk);
        self.minimum_distance_2d.update(chunk);
        self.spheres_3d.update(chunk);
        self.runs_up_down.update(chunk);
        self.operm5.update(chunk);
        self.runs_test.update(chunk);
        self.oqso.update(chunk);
        self.dna.update(chunk);
        self.count_ones_stream.update(chunk);
        self.monobit.update(chunk);
        self.chi_square.update(chunk);
        self.squeeze.update(chunk);
        self.overlapping_sums.update(chunk);
        self.craps.update(chunk);
    }

    pub fn reset(&mut self) {
        self.birthday_spacings.reset();
        self.parking_lot.reset();
        self.minimum_distance_2d.reset();
        self.spheres_3d.reset();
        self.runs_up_down.reset();
        self.operm5.reset();
        self.runs_test.reset();
        self.oqso.reset();
        self.dna.reset();
        self.count_ones_stream.reset();
        self.monobit.reset();
        self.chi_square.reset();
        self.squeeze.reset();
        self.overlapping_sums.reset();
        self.craps.reset();
    }

    /// Evaluates all 12 Dieharder tests and returns a structured [`DieharderEvaluation`].
    pub fn evaluate(&self) -> DieharderEvaluation {
        use randstat_core::traits::TestStatus;

        let entries = [
            DieharderTestEntry {
                name: "Birthday Spacings",
                category: "spatial",
                result: TestResult::NOT_IMPLEMENTED,
            },
            DieharderTestEntry {
                name: "Parking Lot",
                category: "spatial",
                result: TestResult::NOT_IMPLEMENTED,
            },
            DieharderTestEntry {
                name: "Minimum Distance 2D",
                category: "spatial",
                result: TestResult::NOT_IMPLEMENTED,
            },
            DieharderTestEntry {
                name: "3D Spheres",
                category: "spatial",
                result: TestResult::NOT_IMPLEMENTED,
            },
            DieharderTestEntry {
                name: "Runs Up/Down",
                category: "runs",
                result: self.runs_test.evaluate(),
            },
            DieharderTestEntry {
                name: "OPERM5",
                category: "runs",
                result: TestResult::NOT_IMPLEMENTED,
            },
            DieharderTestEntry {
                name: "OQSO",
                category: "template",
                result: TestResult::NOT_IMPLEMENTED,
            },
            DieharderTestEntry {
                name: "DNA",
                category: "template",
                result: TestResult::NOT_IMPLEMENTED,
            },
            DieharderTestEntry {
                name: "Count Ones in Stream",
                category: "frequency",
                result: self.monobit.evaluate(),
            },
            DieharderTestEntry {
                name: "Squeeze",
                category: "complexity",
                result: TestResult::NOT_IMPLEMENTED,
            },
            DieharderTestEntry {
                name: "Overlapping Sums",
                category: "distribution",
                result: self.chi_square.evaluate(),
            },
            DieharderTestEntry {
                name: "Craps",
                category: "distribution",
                result: TestResult::NOT_IMPLEMENTED,
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

        DieharderEvaluation {
            entries,
            total_tests: entries.len(),
            implemented_count,
            passed_count,
            failed_count,
            skipped_count,
        }
    }
}

/// Result entry for an individual Dieharder test.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DieharderTestEntry {
    pub name: &'static str,
    pub category: &'static str,
    pub result: randstat_core::traits::TestResult,
}

/// Aggregated evaluation result for the Dieharder test battery.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DieharderEvaluation {
    pub entries: [DieharderTestEntry; 12],
    pub total_tests: usize,
    pub implemented_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub skipped_count: usize,
}

impl Default for DieharderSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dieharder_suite() {
        let mut suite = DieharderSuite::new();
        let sample = [0xAA; 128];
        suite.update(&sample);
        assert_eq!(suite.count_ones_stream.total_bytes, 128);

        let eval = suite.evaluate();
        assert_eq!(eval.total_tests, 12);
        assert_eq!(eval.implemented_count, 3);
        assert_eq!(eval.skipped_count, 9);

        let mut default_suite = DieharderSuite::default();
        default_suite.update(&[1, 2, 3]);
        default_suite.reset();
        assert_eq!(default_suite.count_ones_stream.total_bytes, 0);
    }
}
