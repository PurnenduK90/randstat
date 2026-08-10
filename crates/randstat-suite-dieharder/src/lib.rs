//! `randstat-suite-dieharder` — Dieharder test suite.
//!
//! Aggregates the 12 Dieharder-specific tests. All tests are currently stubs.
//! Tests are imported from their **functional category** inside `randstat-tests`
//! (not from a `dieharder/` module) so that the categorical taxonomy is the
//! single source of truth for reporting.
//!
//! Reference: <https://webhome.phy.duke.edu/~rgb/General/dieharder.php>
//!
//! | Test | Functional Category | Status |
//! |---|---|---|
//! | Birthday Spacings | `spatial` | 🔧 Stub |
//! | Parking Lot | `spatial` | 🔧 Stub |
//! | Minimum Distance 2D | `spatial` | 🔧 Stub |
//! | 3D Spheres | `spatial` | 🔧 Stub |
//! | Runs Up/Down | `runs` | 🔧 Stub |
//! | OPERM5 | `runs` | 🔧 Stub |
//! | OQSO | `template` | 🔧 Stub |
//! | DNA | `template` | 🔧 Stub |
//! | Count Ones in Stream | `frequency` | 🔧 Stub |
//! | Squeeze | `complexity` | 🔧 Stub |
//! | Overlapping Sums | `distribution` | 🔧 Stub |
//! | Craps | `distribution` | 🔧 Stub |

#![no_std]

use randstat_core::traits::StreamTest;

// Spatial tests
use randstat_tests::spatial::birthday_spacings::BirthdaySpacingsTest;
use randstat_tests::spatial::minimum_distance_2d::MinimumDistance2DTest;
use randstat_tests::spatial::parking_lot::ParkingLotTest;
use randstat_tests::spatial::spheres_3d::Spheres3DTest;

// Runs tests
use randstat_tests::runs::operm5::Operm5Test;
use randstat_tests::runs::runs_up_down::RunsUpDownTest;

// Template / occupancy tests
use randstat_tests::template::dna::DnaTest;
use randstat_tests::template::oqso::OqsoTest;

// Frequency tests
use randstat_tests::frequency::count_ones_stream::CountOnesStreamTest;

// Complexity tests
use randstat_tests::complexity::squeeze::SqueezeTest;

// Distribution tests
use randstat_tests::distribution::craps::CrapsTest;
use randstat_tests::distribution::overlapping_sums::OverlappingSumsTest;

/// Zero-alloc Dieharder-specific suite (12 stubs).
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
    // Template / occupancy
    pub oqso: OqsoTest,
    pub dna: DnaTest,
    // Frequency
    pub count_ones_stream: CountOnesStreamTest,
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
        oqso: OqsoTest::new(),
        dna: DnaTest::new(),
        count_ones_stream: CountOnesStreamTest::new(),
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
        // Spatial
        self.birthday_spacings.update(chunk);
        self.parking_lot.update(chunk);
        self.minimum_distance_2d.update(chunk);
        self.spheres_3d.update(chunk);
        // Runs
        self.runs_up_down.update(chunk);
        self.operm5.update(chunk);
        // Template
        self.oqso.update(chunk);
        self.dna.update(chunk);
        // Frequency
        self.count_ones_stream.update(chunk);
        // Complexity
        self.squeeze.update(chunk);
        // Distribution
        self.overlapping_sums.update(chunk);
        self.craps.update(chunk);
    }

    pub fn reset(&mut self) {
        // Spatial
        self.birthday_spacings.reset();
        self.parking_lot.reset();
        self.minimum_distance_2d.reset();
        self.spheres_3d.reset();
        // Runs
        self.runs_up_down.reset();
        self.operm5.reset();
        // Template
        self.oqso.reset();
        self.dna.reset();
        // Frequency
        self.count_ones_stream.reset();
        // Complexity
        self.squeeze.reset();
        // Distribution
        self.overlapping_sums.reset();
        self.craps.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dieharder_suite() {
        let mut suite = DieharderSuite::new();
        suite.update(&[1, 2, 3, 4, 5]);
        assert_eq!(suite.count_ones_stream.total_bytes, 5);

        let mut default_suite = DieharderSuite::default();
        default_suite.update(&[1, 2, 3]);
        default_suite.reset();
        assert_eq!(default_suite.count_ones_stream.total_bytes, 0);
    }
}

impl Default for DieharderSuite {
    fn default() -> Self {
        Self::new()
    }
}
