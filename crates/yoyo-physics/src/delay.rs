//! Instantaneous movement after a delay.

use num_traits::Float;

use super::{Approximation, Curve};

/// Instantaneous movement after a delay.
#[derive(Copy, Clone, Debug)]
pub struct Delay<T>
where
    T: Float,
{
    /// Start value of the animation.
    pub from_value: T,

    /// Target value of the animation.
    pub to_value: T,

    /// The amount of time to wait before movement.
    pub duration: f32,
}

impl<T> Curve for Delay<T>
where
    T: Float,
{
    type Value = T;
    type Velocity = T;

    fn approximate(&self, time: f32) -> Approximation<T> {
        if time < self.duration {
            Approximation {
                value: self.from_value,
                velocity: T::zero(),
            }
        } else {
            Approximation {
                value: self.to_value,
                velocity: T::zero(),
            }
        }
    }

    fn target(&self) -> T {
        self.to_value
    }
}
