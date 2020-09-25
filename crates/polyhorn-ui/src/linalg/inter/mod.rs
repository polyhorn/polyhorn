//! Types and algorithms that implement perceptually plausible interpolation
//! between 3D transformations.

mod algebra;
mod compose;
mod laplace;

pub use compose::Decomposition3D;
pub use laplace::LaplaceExpansion3D;
