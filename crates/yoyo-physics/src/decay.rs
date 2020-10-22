//! Deceleration towards a target.
//!
//! This implementation is derived from a JavaScript implementation at
//! [https://github.com/framer/motion](https://github.com/framer/motion).

use num_traits::{Float, NumCast};

use super::{Approximation, Curve};

/// Decay curve that starts at a given value and decelerates to a given target
/// value.
#[derive(Copy, Clone, Debug)]
pub struct Decay<T>
where
    T: Float,
{
    /// Start value of the animation.
    pub from_value: T,

    /// Target value of the animation.
    pub to_value: T,

    /// Adjusting the time constant will change the duration of the deceleration,
    /// thereby affecting its feel.
    pub time_constant: f32,
}

impl<T> Decay<T>
where
    T: Float,
{
    /// Computes the ideal target of the decay.
    pub fn ideal_target(from_value: T, power: T, velocity: T) -> T {
        from_value + power * velocity
    }
}

impl<T> Curve for Decay<T>
where
    T: Float,
{
    type Value = T;
    type Velocity = T;

    fn approximate(&self, time: f32) -> Approximation<T> {
        let amplitude = self.to_value - self.from_value;
        let multiplier = <T as NumCast>::from((-time / self.time_constant).exp()).unwrap();
        let delta = -amplitude * multiplier;

        Approximation {
            value: self.to_value + delta,
            velocity: T::zero(),
        }
    }

    fn target(&self) -> T {
        self.to_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ideal_target() {
        // The ideal target is defined as `y_0 + p * v` where `y_0` is the
        // initial value, `p` is power and `v` is velocity.
        assert_eq!(Decay::ideal_target(4.0, 0.8, 32.0), 4.0 + 0.8 * 32.0);
    }
}
