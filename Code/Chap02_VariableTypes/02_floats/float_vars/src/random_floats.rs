use rand::SeedableRng;
use rand::distributions::{Distribution, Uniform};
/// Generates a random single-precision (f32) float number within the specified range.
///
/// # Arguments
///
/// * `seed` - The seed value for the random number generator.
/// * `min` - The minimum value of the range (inclusive).
/// * `max` - The maximum value of the range (inclusive).
///
/// # Returns
///
/// Returns a random f32 float number within the specified range.
/// 
pub fn generate_random_float_f32(seed: u64, min: f32, max: f32) -> f32 {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let range = Uniform::new_inclusive(min, max);
    range.sample(&mut rng)
}
/// Generates a random double-precision (f64) float number within the specified range.
///
/// # Arguments
///
/// * `seed` - The seed value for the random number generator.
/// * `min` - The minimum value of the range (inclusive).
/// * `max` - The maximum value of the range (inclusive).
///
/// # Returns
///
/// Returns a random f64 float number within the specified range.

pub fn generate_random_float_f64(seed: u64, min: f64, max: f64) -> f64 {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let range = Uniform::new_inclusive(min, max);
    range.sample(&mut rng)
}
