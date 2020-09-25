/// Approximation of the value (y-coordinate) and velocity of a curve at the
/// given (unnormalized) time (x-coordinate).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Approximation<T, V = T> {
    /// This is the y-coordinate of an approximation.
    pub value: T,

    /// This is the y-coordinate of the derivative at the x-coordinate of this
    /// approximation.
    pub velocity: V,
}

/// Curves provide the mathematical foundation for animation.
///
/// Within Yoyo, we restrict curves to those with a closed-form approximation /
/// analytical solution and similarly, a closed-form derivative that we can use
/// to compute velocity. Typically, especially for springs, animation frameworks
/// use numerical integrations, like RK4. We want closed-form solutions because
/// they let us pause and resume animations at arbitrary time steps.
pub trait Curve {
    /// This is the type that this curve operates on.
    type Value;

    /// This is the type of velocity that this curve works with.
    type Velocity;

    /// This should return a single approximation for the given time.
    fn approximate(&self, time: f32) -> Approximation<Self::Value, Self::Velocity>;

    /// This should return the target value of this curve.
    fn target(&self) -> Self::Value;
}
