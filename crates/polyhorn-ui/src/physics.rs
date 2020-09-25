//! Primitives for a few physics-based types (velocity, angles).

use num_traits::{Float, FloatConst};
use std::fmt::{Debug, Formatter, Result};
use std::ops::{Add, Div, Sub};
use std::time::Duration;

/// Represents dy/dt. This type is mostly used for events that involve movement,
/// such as scroll events.
pub struct Velocity<T> {
    delta: T,
    time: Duration,
}

impl<T> Velocity<T> {
    /// Returns a new velocity with the given delta within the given timeframe.
    pub fn new(delta: T, time: Duration) -> Velocity<T> {
        Velocity { delta, time }
    }

    /// Returns the velocity in O per second where O is the result of dividing
    /// T by a f32 that represents the duration of this velocity sample.
    pub fn into_per_second<X, O>(self) -> O
    where
        X: From<f32>,
        T: Div<X, Output = O>,
    {
        self.delta / X::from(self.time.as_secs_f32())
    }
}

/// Type-safe representation for angles.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Angle<T> {
    radians: T,
}

impl<T> Angle<T>
where
    T: Float,
{
    /// Returns a new angle with the given radians.
    pub fn with_radians(radians: T) -> Angle<T> {
        Angle { radians }
    }

    /// Converts the given angle to radians.
    pub fn to_radians(&self) -> T {
        self.radians
    }
}

impl<T> Angle<T>
where
    T: Float + FloatConst,
{
    /// Returns a new angle with the given degrees.
    pub fn with_degrees(degrees: T) -> Angle<T> {
        Angle {
            radians: degrees / T::from(180.0).unwrap() * T::PI(),
        }
    }

    /// Converts the given angle to degrees.
    pub fn to_degrees(&self) -> T {
        self.radians / T::PI() * T::from(180.0).unwrap()
    }
}

impl<T> Add for &Angle<T>
where
    T: Float + FloatConst,
{
    type Output = Angle<T>;

    fn add(self, rhs: &Angle<T>) -> Self::Output {
        Angle {
            radians: self.radians + rhs.radians,
        }
    }
}

impl<T> Sub for &Angle<T>
where
    T: Float + FloatConst,
{
    type Output = Angle<T>;

    fn sub(self, rhs: &Angle<T>) -> Self::Output {
        Angle {
            radians: self.radians - rhs.radians,
        }
    }
}

impl<T> Debug for Angle<T>
where
    T: Float + FloatConst + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("{:?}º", self.to_degrees()))
    }
}
