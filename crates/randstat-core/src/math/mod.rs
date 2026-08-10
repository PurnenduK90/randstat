//! Chi-square, Normal distribution, and related mathematical functions.
//!
//! All functions are `#[inline]`-friendly and call only `libm` — no `std` math.

pub mod chi2;
pub mod plot;

// Convenience re-exports for WASM and CLI callers
pub use chi2::{
    chi2_critical_value, chi2_pdf, compute_chi_square, lgamma, normal_pdf, pochisq, poz,
};
pub use plot::{generate_chi2_points, generate_normal_points};
