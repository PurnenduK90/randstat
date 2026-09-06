//! Mathematical transformation algorithms for distributions and bit generators.
//!
//! Exposes pure, stateless mathematical primitives that can be used independently
//! without instantiating generator structs.

use libm::{cos, log, sin, sqrt};

const PI_2: f64 = 2.0 * core::f64::consts::PI;

/// Box-Muller transformation.
///
/// Maps two independent uniform random variables $u_1, u_2 \in (0, 1)$ into two
/// independent standard normal random variables $z_0, z_1 \sim \mathcal{N}(0, 1)$.
#[inline]
pub fn box_muller(mut u1: f64, u2: f64) -> (f64, f64) {
    if u1 < 1e-15 {
        u1 = 1e-15;
    }
    let r = sqrt(-2.0 * log(u1));
    let theta = PI_2 * u2;
    (r * cos(theta), r * sin(theta))
}

/// SplitMix64 PRNG step algorithm.
///
/// Fast, high-entropy 64-bit state mixer used for initializing PRNG states.
#[inline]
pub fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

/// Xoshiro256** single step transition function.
///
/// Updates 4-word state array and returns next 64-bit pseudo-random word.
#[inline]
pub fn xoshiro256_next(s: &mut [u64; 4]) -> u64 {
    let result = (s[1].wrapping_mul(5)).rotate_left(7).wrapping_mul(9);
    let t = s[1] << 17;

    s[2] ^= s[0];
    s[3] ^= s[1];
    s[1] ^= s[2];
    s[0] ^= s[3];

    s[2] ^= t;
    s[3] = s[3].rotate_left(45);

    result
}

/// Pure LFSR step transition function for primitive polynomials of orders 3 to 128.
///
/// Returns `(next_state, output_bit)`.
#[inline]
pub fn lfsr_step_bit(state: u128, order: u32) -> (u128, u8) {
    let bit = (state & 1) as u8;
    let s = state;
    let (fb, high_shift) = match order {
        3 => ((s ^ (s >> 2)) & 1, 2),
        4 => ((s ^ (s >> 3)) & 1, 3),
        8 => ((s ^ (s >> 2) ^ (s >> 3) ^ (s >> 7)) & 1, 7),
        9 => ((s ^ (s >> 5)) & 1, 8),
        11 => ((s ^ (s >> 9)) & 1, 10),
        13 => ((s ^ (s >> 9) ^ (s >> 10) ^ (s >> 12)) & 1, 12),
        19 => ((s ^ (s >> 14) ^ (s >> 17) ^ (s >> 18)) & 1, 18),
        27 => ((s ^ (s >> 22) ^ (s >> 25) ^ (s >> 26)) & 1, 26),
        31 => ((s ^ (s >> 28)) & 1, 30),
        32 => ((s ^ (s >> 10) ^ (s >> 30) ^ (s >> 31)) & 1, 31),
        64 => ((s ^ (s >> 60) ^ (s >> 61) ^ (s >> 63)) & 1, 63),
        96 => ((s ^ (s >> 47) ^ (s >> 49) ^ (s >> 95)) & 1, 95),
        128 => ((s ^ (s >> 99) ^ (s >> 101) ^ (s >> 126)) & 1, 127),
        _ => ((s ^ (s >> 28)) & 1, 30),
    };
    let next_state = (state >> 1) | (fb << high_shift);
    (next_state, bit)
}

/// Pure De Bruijn cycle step transition function for orders 4, 9, 13, 19, 31.
///
/// Returns `(next_state, output_bit)`.
#[inline]
pub fn debruijn_step_bit(state: u32, order: u32) -> (u32, u8) {
    let bit = (state & 1) as u8;
    let s = state;
    let is_bridge = (s == 1) || (s == 0);

    let (mut fb, high_shift) = match order {
        4 => (((s ^ (s >> 3)) & 1), 3),
        9 => (((s ^ (s >> 5)) & 1), 8),
        13 => (((s ^ (s >> 9) ^ (s >> 10) ^ (s >> 12)) & 1), 12),
        19 => (((s ^ (s >> 14) ^ (s >> 17) ^ (s >> 18)) & 1), 18),
        31 => (((s ^ (s >> 28)) & 1), 30),
        _ => (((s ^ (s >> 14) ^ (s >> 17) ^ (s >> 18)) & 1), 18),
    };

    if is_bridge {
        fb ^= 1;
    }

    let next_state = (state >> 1) | (fb << high_shift);
    (next_state, bit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_muller() {
        let (z0, z1) = box_muller(0.5, 0.25);
        assert!(z0.is_finite());
        assert!(z1.is_finite());
    }

    #[test]
    fn test_splitmix_and_xoshiro() {
        let mut seed = 42u64;
        let mut s = [
            splitmix64(&mut seed),
            splitmix64(&mut seed),
            splitmix64(&mut seed),
            splitmix64(&mut seed),
        ];
        let val = xoshiro256_next(&mut s);
        assert_ne!(val, 0);
    }

    #[test]
    fn test_lfsr_step() {
        let (next_s, bit) = lfsr_step_bit(1, 4);
        assert_eq!(bit, 1);
        assert_ne!(next_s, 1);
    }
}
