//! `randstat-suite-full` — Comprehensive meta-suite.
//!
//! Aggregates all standard batteries into a single struct, fanning out
//! `update` to all. Use this for comprehensive evaluations where binary size
//! is not a constraint.
//!
//! This is a **meta-suite** — it holds sub-suites and delegates to them.
//! No test logic is duplicated.

#![no_std]

use randstat_suite_ais31::{Ais31Evaluation, Ais31Suite};
use randstat_suite_dieharder::{DieharderEvaluation, DieharderSuite};
use randstat_suite_ent::EntSuite;
use randstat_suite_gjrand::{GjrandEvaluation, GjrandSuite};
use randstat_suite_nist::{NistEvaluation, NistSuite};
use randstat_suite_practrand::{PractRandEvaluation, PractRandSuite};
use randstat_suite_sp800_90b::{Sp80090bEvaluation, Sp80090bSuite};
use randstat_suite_testu01::{TestU01Evaluation, TestU01Suite};

/// Zero-alloc comprehensive meta-suite aggregating all statistical batteries.
#[derive(Debug, Clone, Copy)]
pub struct FullSuite {
    pub ent: EntSuite,
    pub nist: NistSuite,
    pub sp800_90b: Sp80090bSuite,
    pub ais31: Ais31Suite,
    pub dieharder: DieharderSuite,
    pub testu01: TestU01Suite,
    pub practrand: PractRandSuite,
    pub gjrand: GjrandSuite,
}

impl FullSuite {
    pub const ZERO: Self = Self {
        ent: EntSuite::ZERO,
        nist: NistSuite::ZERO,
        sp800_90b: Sp80090bSuite::ZERO,
        ais31: Ais31Suite::ZERO,
        dieharder: DieharderSuite::ZERO,
        testu01: TestU01Suite::ZERO,
        practrand: PractRandSuite::ZERO,
        gjrand: GjrandSuite::ZERO,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.ent.update(chunk);
        self.nist.update(chunk);
        self.sp800_90b.update(chunk);
        self.ais31.update(chunk);
        self.dieharder.update(chunk);
        self.testu01.update(chunk);
        self.practrand.update(chunk);
        self.gjrand.update(chunk);
    }

    pub fn reset(&mut self) {
        self.ent.reset();
        self.nist.reset();
        self.sp800_90b.reset();
        self.ais31.reset();
        self.dieharder.reset();
        self.testu01.reset();
        self.practrand.reset();
        self.gjrand.reset();
    }

    pub fn evaluate(&self) -> FullEvaluation {
        FullEvaluation {
            nist: self.nist.evaluate(),
            ais31: self.ais31.evaluate(),
            sp800_90b: self.sp800_90b.evaluate(),
            dieharder: self.dieharder.evaluate(),
            testu01: self.testu01.evaluate(),
            practrand: self.practrand.evaluate(),
            gjrand: self.gjrand.evaluate(),
        }
    }
}

/// Aggregated evaluation result for the full battery suite.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FullEvaluation {
    pub nist: NistEvaluation,
    pub ais31: Ais31Evaluation,
    pub sp800_90b: Sp80090bEvaluation,
    pub dieharder: DieharderEvaluation,
    pub testu01: TestU01Evaluation,
    pub practrand: PractRandEvaluation,
    pub gjrand: GjrandEvaluation,
}

impl Default for FullSuite {
    fn default() -> Self {
        Self::new()
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

        let eval = suite.evaluate();
        assert_eq!(eval.nist.total_tests, 15);
        assert_eq!(eval.ais31.total_tests, 9);
        assert_eq!(eval.gjrand.total_tests, 10);
        assert_eq!(eval.practrand.total_tests, 10);

        let mut default_suite = FullSuite::default();
        default_suite.update(&[1, 2, 3]);
        default_suite.reset();
        assert_eq!(default_suite.nist.monobit.total_bits, 0);
    }
}
