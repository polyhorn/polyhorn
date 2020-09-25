/// Approximation of the value (y-coordinate) and velocity of a curve at the
/// given (unnormalized) time (x-coordinate).
pub struct Approximation<T> {
    pub time: T,
    pub value: T,
    pub velocity: T,
}

/// Curves provide the mathematical foundation for animation.
///
/// Within Yoyo, we restrict curves to those with a closed-form approximation /
/// analytical solution and similarly, a closed-form derivative that we can use
/// to compute velocity. Typically, especially for springs, animation frameworks
/// use numerical integrations, like RK4. We want closed-form solutions because
/// they let us pause and resume animations at arbitrary time steps.
pub trait Curve<T> {
    /// This should return a single approximation for the given time.
    fn approximate(&self, time: T) -> Approximation<T>;

    /// This should return the target value of this curve.
    fn target(&self) -> T;
}
