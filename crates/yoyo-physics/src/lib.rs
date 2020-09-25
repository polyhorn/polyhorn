#![warn(missing_docs)]

//! This library implements the basic physics-based primitives we use in Yoyo.
//! The most important task of this library is to support continuous animation.
//!
//! Continuous animation is when a running, uncompleted animation with a
//! particular value and velocity is replaced with a new animation that starts
//! from that same value and velocity so that it is seemingly one and the same
//! animation to the user.
//!
//! Example
//! -------
//! ```rust
//! use yoyo_physics::Curve;
//! use yoyo_physics::bezier::Bezier;
//! use yoyo_physics::spring::Spring;
//!
//! // Start a Bezier with strong easing.
//! let bezier = Bezier {
//!     from_value: 0.0,
//!     to_value: 320.0,
//!     duration: 1.0,
//!     control_points: [(0.95, 0.05), (0.05, 0.95)],
//! };
//!
//! // Pause the Bezier midway.
//! let sample = bezier.approximate(0.5);
//!
//! // Start a spring from the Bezier's state.
//! let spring = Spring {
//!     from_value: sample.value,
//!     to_value: 320.0,
//!     initial_velocity: sample.velocity,
//!     ..Default::default()
//! };
//!
//! // Verify that the spring's starting velocity equals the Bezier's midway
//! // velocity.
//! assert_eq!(spring.approximate(0.0).velocity, sample.velocity);
//! ```

pub mod bezier;
pub mod decay;
pub mod delay;
pub mod spring;
pub mod threshold;

mod curve;
mod sampler;

pub use curve::{Approximation, Curve};
pub use sampler::{KeyedIter, Sampler};
pub use threshold::Threshold;
