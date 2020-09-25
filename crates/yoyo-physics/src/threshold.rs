//! Threshold traits and types that are used to stop a pending animation.

use num::Float;

use super::Approximation;

/// Implemented by types that can evaluate an approximation and determine if it
/// is resting.
pub trait Threshold {
    /// The type of value that this threshold applies to.
    type Value;

    /// The type of velocity that this threshold applies to.
    type Velocity;

    /// This function should evaluate the given approximate and return a boolean
    /// that indicates if the threshold for resting is met.
    fn evaluate(&mut self, approximation: &Approximation<Self::Value, Self::Velocity>) -> bool;
}

/// This threshold evaluates to true if both of the given thresholds evaluate to
/// true.
pub struct And<A, B>(pub A, pub B)
where
    A: Threshold,
    B: Threshold<Value = A::Value>;

impl<A, B> Threshold for And<A, B>
where
    A: Threshold,
    B: Threshold<Value = A::Value, Velocity = A::Velocity>,
{
    type Value = A::Value;
    type Velocity = A::Velocity;

    fn evaluate(&mut self, approximation: &Approximation<Self::Value, Self::Velocity>) -> bool {
        self.0.evaluate(approximation) && self.1.evaluate(approximation)
    }
}

/// This threshold evaluates to true if the displacement of a running animation
/// drops below the given threshold value.
pub struct DisplacementThreshold<T>
where
    T: Float,
{
    /// This is the target value of an animation.
    pub target: T,

    /// This is the sensitivity of this threshold.
    pub sensitivity: T,
}

impl<T> Threshold for DisplacementThreshold<T>
where
    T: Float,
{
    type Value = T;
    type Velocity = T;

    fn evaluate(&mut self, approximation: &Approximation<Self::Value, Self::Velocity>) -> bool {
        let displacement = approximation.value.sub(self.target);

        displacement.abs() <= self.sensitivity
    }
}

/// This threshold evaluates to true if the displacement of a running animation
/// drops below the given threshold value.
pub struct VelocityThreshold<T>(pub T);

impl<T> Threshold for VelocityThreshold<T>
where
    T: Float,
{
    type Value = T;
    type Velocity = T;

    fn evaluate(&mut self, approximation: &Approximation<Self::Value, Self::Velocity>) -> bool {
        approximation.velocity.abs() <= self.0
    }
}
