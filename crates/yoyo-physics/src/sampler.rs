use num_traits::{Float, NumCast};

use super::{Approximation, Curve, Threshold};
use crate::threshold::{And, DisplacementThreshold, VelocityThreshold};

/// Sampling iterator over curves that generates approximations at a fixed sample
/// rate.
#[derive(Copy, Clone)]
pub struct Sampler<'a, C, T>
where
    C: Curve + ?Sized,
    T: Threshold,
{
    /// This is the spring that we want to sample.
    curve: &'a C,

    /// This is the sample rate (i.e. how many samples per second).
    sample_rate: f32,

    threshold: T,

    /// This is the current time step of the iterator.
    time: f32,

    /// This is a boolean that indicates if the sampler is exhausted (i.e. both
    /// thresholds have been met at the same time).
    exhausted: bool,
}

impl<'a, C, T> Sampler<'a, C, T>
where
    C: Curve + ?Sized,
    T: Threshold<Value = C::Value>,
{
    /// This function creates a new sampler of the given curve with the given
    /// sample rate. The sample rate is the number of samples per second. This
    /// will use the default thresholds of `1e-3`.
    pub fn with_threshold(curve: &'a C, sample_rate: f32, threshold: T) -> Sampler<'a, C, T> {
        Sampler {
            curve,
            sample_rate,
            threshold,
            time: 0.0,
            exhausted: false,
        }
    }

    /// Turns this sampler into a keyed iterator, which returns the timestamps
    /// along with the approximations.
    pub fn keyed(self) -> KeyedIter<'a, C, T> {
        KeyedIter(self)
    }
}

impl<'a, C, T> Sampler<'a, C, And<VelocityThreshold<T>, DisplacementThreshold<T>>>
where
    C: Curve<Value = T> + ?Sized,
    T: Float,
{
    /// This function creates a new sampler of the given curve with the given
    /// sample rate. The sample rate is the number of samples per second. This
    /// will use the default thresholds of `1e-3`.
    pub fn new(
        curve: &'a C,
        sample_rate: f32,
    ) -> Sampler<'a, C, And<VelocityThreshold<T>, DisplacementThreshold<T>>> {
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
        sample_rate: f32,
        rest_velocity_threshold: T,
        rest_displacement_threshold: T,
    ) -> Sampler<'a, C, And<VelocityThreshold<T>, DisplacementThreshold<T>>> {
        Sampler::with_threshold(
            curve,
            sample_rate,
            And(
                VelocityThreshold(rest_velocity_threshold),
                DisplacementThreshold {
                    target: curve.target(),
                    sensitivity: rest_displacement_threshold,
                },
            ),
        )
    }
}

impl<'a, C, T> Iterator for Sampler<'a, C, T>
where
    C: Curve + ?Sized,
    T: Threshold<Value = C::Value, Velocity = C::Velocity>,
{
    type Item = Approximation<C::Value, C::Velocity>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        let mut approx = self.curve.approximate(self.time);
        self.time = self.time + 1.0 / self.sample_rate;

        // If both thresholds are met, we still return this approximation
        // (because we're still interested in its velocity) but we set a flag in
        // the sampler state to prevent any further samples.
        if self.threshold.evaluate(&approx) {
            self.exhausted = true;

            approx.value = self.curve.target();
        }

        Some(approx)
    }
}

/// Iterator that yields both the timestamps of each approximation as the
/// approximation itself.
pub struct KeyedIter<'a, C, T>(Sampler<'a, C, T>)
where
    C: Curve + ?Sized,
    T: Threshold<Value = C::Value>;

impl<'a, C, T> Iterator for KeyedIter<'a, C, T>
where
    C: Curve + ?Sized,
    T: Threshold<Value = C::Value, Velocity = C::Velocity>,
{
    type Item = (f32, Approximation<C::Value, C::Velocity>);

    fn next(&mut self) -> Option<Self::Item> {
        let time = self.0.time;
        Some((time, self.0.next()?))
    }
}
