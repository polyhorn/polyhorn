//! A bunch of linear algebra that is shared between platform implementations
//! and used to compute and interpolate 3D transforms.

mod geometry;
pub mod inter;
mod transform;

#[cfg(test)]
mod tests;

pub use geometry::{Point3D, Quaternion3D};
pub use transform::Transform3D;
