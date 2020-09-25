use super::{Approximation, Curve};
use num::Float;

#[derive(Copy, Clone, Debug)]
pub struct Delay<T>
where
    T: Float,
{
    /// Start value of the animation.
    pub from_value: T,

    /// Target value of the animation.
    pub to_value: T,

    pub duration: T,
}

impl<T> Curve<T> for Delay<T>
where
    T: Float,
{
    fn approximate(&self, time: T) -> Approximation<T> {
        if time < self.duration {
            Approximation {
                time,
                value: self.from_value,
                velocity: T::zero(),
            }
        } else {
            Approximation {
                time,
                value: self.to_value,
                velocity: T::zero(),
            }
        }
    }

    fn target(&self) -> T {
        self.to_value
    }
}
