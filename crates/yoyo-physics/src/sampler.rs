use super::{Approximation, Curve};
use num::{Float, NumCast};

/// Sampling iterator over curves that generates approximations at a fixed sample
/// rate.
#[derive(Copy, Clone)]
pub struct Sampler<'a, C, T>
where
    C: Curve<T> + ?Sized,
    T: Float,
{
    /// This is the spring that we want to sample.
    curve: &'a C,

    /// This is the sample rate (i.e. how many samples per second).
    sample_rate: T,

    /// This is the current time step of the iterator.
    time: T,

    /// This is a boolean that indicates if the sampler is exhausted (i.e. both
    /// thresholds have been met at the same time).
    exhausted: bool,

    /// When spring's velocity is below `rest_velocity_threshold`, it is at rest.
    rest_velocity_threshold: T,

    /// When the spring's displacement (current value) is below
    /// `rest_displacement_threshold`, it is at rest.
    rest_displacement_threshold: T,
}

impl<'a, C, T> Sampler<'a, C, T>
where
    C: Curve<T> + ?Sized,
    T: Float,
{
    /// This function creates a new sampler of the given curve with the given
    /// sample rate. The sample rate is the number of samples per second. This
    /// will use the default thresholds of `1e-3`.
    pub fn new(curve: &'a C, sample_rate: T) -> Sampler<'a, C, T> {
        Sampler::with_thresholds(
            curve,
            sample_rate,
            <T as NumCast>::from(0.001).unwrap(),
            <T as NumCast>::from(0.001).unwrap(),
        )
    }

    /// This function creates a new sampler of the given curve with the given
    /// sample rate and thresholds. The sample rate is the number of samples per
    /// second. Both thresholds must be met in order for the sampler to rest. The
    /// velocity threshold applies to the absolute velocity at a given time. The
    /// displacement threshold applies to the absolute distance between the
    /// current value at a given time and the target value.
    pub fn with_thresholds(
        curve: &'a C,
        sample_rate: T,
        rest_velocity_threshold: T,
        rest_displacement_threshold: T,
    ) -> Sampler<'a, C, T> {
        Sampler {
            curve,
            sample_rate,
            time: T::zero(),
            exhausted: false,
            rest_velocity_threshold,
            rest_displacement_threshold,
        }
    }
}

impl<'a, C, T> Iterator for Sampler<'a, C, T>
where
    C: Curve<T> + ?Sized,
    T: Float,
{
    type Item = Approximation<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        let mut approx = self.curve.approximate(self.time);
        self.time = self.time + T::one() / self.sample_rate;

        let displacement = approx.value - self.curve.target();

        // If both thresholds are met, we still return this approximation
        // (because we're still interested in its velocity) but we set a flag in
        // the sampler state to prevent any further samples.
        if approx.velocity.abs() <= self.rest_velocity_threshold
            && displacement.abs() <= self.rest_displacement_threshold
        {
            self.exhausted = true;

            approx.value = self.curve.target();
        }

        Some(approx)
    }
}
