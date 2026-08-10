//! `randstat-tests` — All 33 individual statistical test accumulators.
//!
//! Every test lives in its own dedicated source file ("One Test, One File").
//! Tests implement [`randstat_core::traits::StreamTest`] and are thin wrappers
//! that delegate evaluation to pure functions in `randstat_core::algorithms`.

#![no_std]

pub mod complexity;
pub mod distribution;
pub mod excursions;
pub mod frequency;
pub mod matrix;
pub mod runs;
pub mod spatial;
pub mod spectral;
pub mod template;

#[cfg(test)]
mod tests {
    use randstat_core::traits::StreamTest;

    fn test_one<T: StreamTest + Default>() {
        let mut t = T::default();
        t.update(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let res = t.evaluate();
        assert!(res.p_value >= 0.0);
        t.reset();
    }

    #[test]
    fn test_all_accumulators() {
        test_one::<super::frequency::arithmetic_mean::ArithmeticMeanTest>();
        test_one::<super::frequency::block_frequency::BlockFrequencyTest>();
        test_one::<super::frequency::chi_square::ChiSquareTest>();
        test_one::<super::frequency::count_ones_stream::CountOnesStreamTest>();
        test_one::<super::frequency::cusum::CusumTest>();
        test_one::<super::frequency::monobit::MonobitTest>();
        test_one::<super::frequency::serial_test::SerialTest>();
        test_one::<super::frequency::shannon_entropy::ShannonEntropyTest>();

        test_one::<super::runs::longest_run::LongestRunTest>();
        test_one::<super::runs::operm5::Operm5Test>();
        test_one::<super::runs::runs_test::RunsTest>();
        test_one::<super::runs::runs_up_down::RunsUpDownTest>();

        test_one::<super::spectral::dft_fft::DftTest>();

        test_one::<super::template::dna::DnaTest>();
        test_one::<super::template::non_overlapping_template::NonOverlappingTemplateTest>();
        test_one::<super::template::oqso::OqsoTest>();
        test_one::<super::template::overlapping_template::OverlappingTemplateTest>();

        test_one::<super::complexity::approx_entropy::ApproxEntropyTest>();
        test_one::<super::complexity::berlekamp_massey::BerlekampMasseyTest>();
        test_one::<super::complexity::maurers_universal::MaurersUniversalTest>();
        test_one::<super::complexity::squeeze::SqueezeTest>();

        test_one::<super::spatial::birthday_spacings::BirthdaySpacingsTest>();
        test_one::<super::spatial::minimum_distance_2d::MinimumDistance2DTest>();
        test_one::<super::spatial::monte_carlo_pi::MonteCarloPiTest>();
        test_one::<super::spatial::parking_lot::ParkingLotTest>();
        test_one::<super::spatial::serial_correlation::SerialCorrelationTest>();
        test_one::<super::spatial::spheres_3d::Spheres3DTest>();

        test_one::<super::matrix::binary_matrix_rank::BinaryMatrixRankTest>();

        test_one::<super::excursions::random_excursions::RandomExcursionsTest>();
        test_one::<super::excursions::random_excursions_variant::RandomExcursionsVariantTest>();

        test_one::<super::distribution::craps::CrapsTest>();
        test_one::<super::distribution::overlapping_sums::OverlappingSumsTest>();
    }
}
