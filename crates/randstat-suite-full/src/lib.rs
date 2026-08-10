//! `randstat-suite-full` — Comprehensive meta-suite.
//!
//! Aggregates [`EntSuite`] and [`NistSuite`] into a single struct, fanning out
//! `update` to both. Use this for full desktop evaluations where binary size
//! is not a constraint.
//!
//! This is a **meta-suite** — it holds both sub-suites and delegates to them.
//! No test logic is duplicated.

#![no_std]

use randstat_suite_ent::EntSuite;
use randstat_suite_nist::NistSuite;

/// Zero-alloc comprehensive suite: ENT + all NIST tests.
#[derive(Debug, Clone, Copy)]
pub struct FullSuite {
    pub ent: EntSuite,
    pub nist: NistSuite,
}

impl FullSuite {
    pub const ZERO: Self = Self {
        ent: EntSuite::ZERO,
        nist: NistSuite::ZERO,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.ent.update(chunk);
        self.nist.update(chunk);
    }

    pub fn reset(&mut self) {
        self.ent.reset();
        self.nist.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_suite() {
        let mut suite = FullSuite::new();
        suite.update(&[1, 2, 3, 4, 5]);
        assert_eq!(suite.nist.monobit.total_bits, 40);

        let mut default_suite = FullSuite::default();
        default_suite.update(&[1, 2, 3]);
        default_suite.reset();
        assert_eq!(default_suite.nist.monobit.total_bits, 0);
    }
}

impl Default for FullSuite {
    fn default() -> Self {
        Self::new()
    }
}
