//! Damped harmonic oscillator (DHO) suitable for animation.
//!
//! This implementation is derived from a JavaScript implementation at
//! [https://github.com/skevy/wobble](https://github.com/skevy/wobble). This is
//! the same implementation that is used by React Native. The only difference is
//! that we measure time in seconds, whereas in JavaScript time is measured in
//! milliseconds.

use super::{Approximation, Curve};
use num::{Float, NumCast};

/// This is a spring whose oscillation and velocity we can efficiently
/// approximate.
#[derive(Copy, Clone, Debug)]
pub struct Spring<T>
where
    T: Float,
{
    /// Starting value of the animation.
    pub from_value: T,

    /// Ending value of the animation.
    pub to_value: T,

    /// The spring stiffness coefficient.
    pub stiffness: T,

    /// Defines how the spring's motion should be damped due to the forces of
    /// friction.
    pub damping: T,

    /// The mass of the object attached to the end of the spring.
    pub mass: T,

    /// The initial velocity (in units/ms) of the object attached to the spring.
    pub initial_velocity: T,

    /// Whether or not the spring allows "overdamping" (a damping ratio > 1).
    pub allows_overdamping: bool,

    /// False when overshooting is allowed, true when it is not.
    pub overshoot_clamping: bool,
}

impl<T> Default for Spring<T>
where
    T: Float,
{
    fn default() -> Self {
        Spring {
            from_value: T::zero(),
            to_value: T::zero(),
            stiffness: <T as NumCast>::from(100.0).unwrap(),
            damping: <T as NumCast>::from(10.0).unwrap(),
            mass: <T as NumCast>::from(1.0).unwrap(),
            initial_velocity: <T as NumCast>::from(1.0).unwrap(),
            allows_overdamping: false,
            overshoot_clamping: false,
        }
    }
}

impl<T> Curve<T> for Spring<T>
where
    T: Float,
{
    /// This function approximates a spring's oscillation and velocity at the
    /// given timestamp.
    fn approximate(&self, time: T) -> Approximation<T> {
        let time = time * <T as NumCast>::from(1000.0).unwrap();

        let c = self.damping;
        let m = self.mass;
        let k = self.stiffness;
        let from_value = self.from_value;
        let to_value = self.to_value;
        let v0 = -self.initial_velocity;

        assert!(m > T::zero(), "Mass value must be greater than 0.");
        assert!(k > T::zero(), "Stiffness value must be greater than 0.");
        assert!(c > T::zero(), "Damping value must be greater than 0.");

        // Damping ratio (dimensionless).
        let mut zeta = c / (<T as NumCast>::from(2.0).unwrap() * (k * m).sqrt());

        // Undamped angular frequency of the oscillator (rad/ms).
        let omega0 = (k / m).sqrt() / <T as NumCast>::from(1000.0).unwrap();

        // Exponential decay.
        let omega1 = omega0 * (T::one() - zeta * zeta).sqrt();

        // Frequency of damped oscillation.
        let omega2 = omega0 * (zeta * zeta - T::one()).sqrt();

        // Initial displacement of the spring at t = 0.
        let x0 = to_value - from_value;

        // Disable overdamping if requested.
        if zeta > T::one() && !self.allows_overdamping {
            zeta = T::one();
        }

        if zeta < T::one() {
            // Under damped.
            let envelope = (-zeta * omega0 * time).exp();

            let oscillation = to_value
                - envelope
                    * ((v0 + zeta * omega0 * x0) / omega1 * (omega1 * time).sin()
                        + x0 * (omega1 * time).cos());

            // This looks crazy -- it's actually just the derivative of the
            // oscillation function.
            let velocity = zeta
                * omega0
                * envelope
                * ((omega1 * time).sin() * (v0 + zeta * omega0 * x0) / omega1
                    + x0 * (omega1 * time).cos())
                - envelope
                    * ((omega1 * time).cos() * (v0 + zeta * omega0 * x0)
                        - omega1 * x0 * (omega1 * time).sin());

            Approximation {
                time: time / <T as NumCast>::from(1000.0).unwrap(),
                value: oscillation,
                velocity,
            }
        } else if zeta == T::one() {
            // Critically damped.
            let envelope = (-omega0 * time).exp();
            let oscillation = to_value - envelope * (x0 + (v0 + omega0 * x0) * time);
            let velocity =
                envelope * (v0 * (time * omega0 - T::one()) + time * x0 * (omega0 * omega0));

            Approximation {
                time: time / <T as NumCast>::from(1000.0).unwrap(),
                value: oscillation,
                velocity,
            }
        } else {
            // Overdamped.
            let envelope = (-zeta * omega0 * time).exp();
            let oscillation = to_value
                - envelope
                    * ((v0 + zeta * omega0 * x0) * (omega2 * time).sinh()
                        + omega2 * x0 * (omega2 * time).cosh())
                    / omega2;
            let velocity = envelope
                * zeta
                * omega0
                * ((omega2 * time).sinh() * (v0 + zeta * omega0 * x0)
                    + x0 * omega2 * (omega2 * time).cosh())
                / omega2
                - envelope
                    * (omega2 * (omega2 * time).cosh() * (v0 + zeta * omega0 * x0)
                        + omega2 * omega2 * x0 * (omega2 * time).sinh())
                    / omega2;

            Approximation {
                time: time / <T as NumCast>::from(1000.0).unwrap(),
                value: oscillation,
                velocity,
            }
        }
    }

    fn target(&self) -> T {
        self.to_value
    }
}

#[cfg(test)]
mod tests {
    use super::super::Sampler;
    use super::*;

    #[test]
    fn test_spring() {
        let spring = Spring {
            from_value: 0.0,
            to_value: 320.0,
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            initial_velocity: 0.0,
            overshoot_clamping: false,
            allows_overdamping: false,
        };

        let y = Sampler::new(&spring, 20.0)
            .map(|approx| approx.value)
            .collect::<Vec<_>>();

        println!("y: {:?}", y);
        println!("y: {:?}", y.len());
    }
}
