//! Cubic Bezier curves (incl. root finding) suitable for animation.

use num::{Float, NumCast};

use super::{Approximation, Curve};

/// Cubic Bezier curve that is scaled to the given domain and range.
#[derive(Copy, Clone)]
pub struct Bezier<T>
where
    T: Float,
{
    /// This is the start coordinate of the range of the Bezier curve.
    pub from_value: T,

    /// This is the end coordinate of the range of the Bezier curve.
    pub to_value: T,

    /// This is the scale of the domain of the Bezier curve.
    pub duration: T,

    /// These are the two dynamic control points (`P_1` and `P_2`). The first and
    /// last control points (`P_0` and `P_3`) are fixed to `(0, 0)` and `(1, 1)`
    /// respectively.
    pub control_points: [(T, T); 2],
}

/// Polynomial coefficients of a 2D cubic Bezier curve.
///
/// In general, a cubic Bezier curve can be represented by
/// `x = a * t^3 + b * t^2 + c * t` (where `x`, `a`, `b` and `c` are vectors).
pub struct BezierCoefficients<T>
where
    T: Float,
{
    /// This is the third and last coefficient (excluding the constant).
    pub c: (T, T),

    /// This is the second coefficient.
    pub b: (T, T),

    /// This is the first coefficient.
    pub a: (T, T),
}

impl<T> BezierCoefficients<T>
where
    T: Float,
{
    /// This function precomputes the coefficients of a 3rd degree polynomial
    /// for a cubic Bezier curve.
    pub fn new(points: [(T, T); 2]) -> BezierCoefficients<T> {
        let one = T::one();
        let two = one + one;
        let three = one + two;

        // We have to move the first control point's coordinates to be strictly
        // greater than 0 and strictly less than 1, otherwise we end up with
        // some undefined behavior including:
        //
        // 1. what's the derivative at 0 if the polynomial coefficient `c` is
        //    zero?
        // 2. what happens if the derivative of `x` with respect to `t` is
        //    negative (i.e. moving backwards in time)?
        // 3. what happens if the derivative of `x` with respect to `t` is zero
        //    (i.e. the velocity is infinite)?
        //
        // These issues become particular problematic when doing "continuous
        // animations", where a Bezier animation is stopped, its velocity
        // estimated and plugged into a Spring or Inertia animation that replaces
        // the original estimation. Both of those will fail if the initial
        // velocity is infinite (or worse: NaN).
        let alpha = <T as NumCast>::from(0.001).unwrap();
        let beta = one - alpha;

        let points = [
            (
                points[0].0.max(alpha).min(beta),
                points[0].1.max(alpha).min(beta),
            ),
            (
                points[1].0.max(alpha).min(beta),
                points[1].1.max(alpha).min(beta),
            ),
        ];

        let c = (three * points[0].0, three * points[0].1);

        let b = (
            three * (points[1].0 - points[0].0) - c.0,
            three * (points[1].1 - points[0].1) - c.1,
        );

        let a = (one - c.0 - b.0, one - c.1 - b.1);

        BezierCoefficients { c, b, a }
    }

    /// This function computes the `x` coordinate for a given `t` argument.
    /// Specifically, this is: `(x, _) = f(t)` (note that `f` is a parametric
    /// function).
    pub fn sample_x(&self, t: T) -> T {
        ((self.a.0 * t + self.b.0) * t + self.c.0) * t
    }

    /// This function computes the `y` coordinate for a given `t` argument.
    /// Specifically, this is: `(_, y) = f(t)` (note that `f` is a parametric
    /// function).
    pub fn sample_y(&self, t: T) -> T {
        ((self.a.1 * t + self.b.1) * t + self.c.1) * t
    }

    /// This function computes the derivative of the `x` coordinate with respect
    /// to `t` for a given `t` argument. Specifically, this is: `(x', _) = f'(t)`
    /// (note that `f` is a parametric function).
    pub fn sample_dxdt(&self, t: T) -> T {
        let one = T::one();
        let two = one + one;
        let three = one + two;

        (three * self.a.0 * t + two * self.b.0) * t + self.c.0
    }

    /// This function computes the derivative of the `y` coordinate with respect
    /// to `t` for a given `t` argument. Specifically, this is: `(_, y') = f'(t)`
    /// (note that `f` is a parametric function).
    pub fn sample_dydt(&self, t: T) -> T {
        let one = T::one();
        let two = one + one;
        let three = one + two;

        (three * self.a.1 * t + two * self.b.1) * t + self.c.1
    }

    /// This function computes the derivative of the `y` coordinate with respect
    /// to `x` for a given `t` argument.
    pub fn sample_dydx(&self, t: T) -> T {
        let dxdt = self.sample_dxdt(t);

        self.sample_dydt(t) / dxdt
    }

    /// This function solves `(x, _) = f(t)` for a given `t`.
    fn solve_x(&self, x: T, eps: T) -> T {
        let two = T::one() + T::one();

        // The curve is only defined for `0.0 <= t <= 1.0` (the curve domain).
        // However, we also clamp `x` (our actual time domain) because we are
        // not interested in anything that happens before `(0, 0)` or after `(1,
        // 1)` (the fixed control points).
        let x = x.max(T::zero()).min(T::one());

        let mut t2 = x;

        // We start by performing a few iterations of Newton's method. This is
        // what WebKit / Blink do too. This implementation is essentially just
        // gradient descent.
        for _ in 0..8 {
            let x2 = self.sample_x(t2) - x;

            if x2.abs() < eps {
                return t2;
            }

            let d2 = self.sample_dxdt(t2);

            if d2.abs() < eps {
                break;
            }

            t2 = t2 - x2 / d2;
        }

        // If we didn't find `t`, we use a primitive bisection loop, which is
        // bound to be slower.
        let mut t0 = T::zero();
        let mut t1 = T::one();
        let mut t2 = x;

        while t0 < t1 {
            let x2 = self.sample_x(t2);

            if (x2 - x).abs() < eps {
                return t2;
            }

            if x > x2 {
                t0 = t2;
            } else {
                t1 = t2;
            }

            t2 = (t1 - t0) / two + t0;
        }

        t2
    }
}

impl<T> Bezier<T>
where
    T: Float,
{
    /// This function returns the 3rd degree polynomial coefficients for this
    /// cubic Bezier curve.
    fn coefficients(&self) -> BezierCoefficients<T> {
        BezierCoefficients::new(self.control_points)
    }
}

impl<T> Curve<T> for Bezier<T>
where
    T: Float,
{
    fn approximate(&self, time: T) -> Approximation<T> {
        let coeffs = self.coefficients();

        let x = (time / self.duration).max(T::zero()).min(T::one());
        let t = coeffs.solve_x(x, <T as NumCast>::from(0.001).unwrap());

        let delta = self.to_value - self.from_value;

        Approximation {
            time,
            value: coeffs.sample_y(t) * delta + self.from_value,
            velocity: if x.is_one() {
                T::zero()
            } else {
                coeffs.sample_dydx(t) * delta
            },
        }
    }

    fn target(&self) -> T {
        self.to_value
    }
}

#[cfg(test)]
mod tests {
    use super::BezierCoefficients;

    #[test]
    fn test_2nd_derivative() {
        // This is B(0.0, 0.0, 1.0, 1.0) (in 1D). Calculating the derivative of
        // `y` with respect to `x` requires evaluating the derivative of `x` with
        // respect to `t` at 0, which is 0. In order to evaluate the limit of `y`
        // with respect to `x`, we apply L'Hôpital's rule. With this, we find
        // that the derivative at 0 is indeed 1 (i.e. linear).
        let coeffs = BezierCoefficients::new([(0.0, 0.0), (1.0, 1.0)]);
        assert_eq!((coeffs.sample_dydx(0.0f32) * 1000.0).round() / 1000.0, 1.0);
        assert_eq!((coeffs.sample_dydx(0.5f32) * 1000.0).round() / 1000.0, 1.0);
    }

    #[test]
    fn test_3rd_derivative() {
        // TODO: the second application of L'Hôpital's rule does not yet give the
        // results we expect.

        // This is B(0.0, 0.0, 0.0, 1.0) (in 1D). Calculating the derivative of
        // `y` with respect to `x` requires evaluating the derivative of `x` with
        // respect to `t` at 0, which is 0. In order to evaluate the limit of `y`
        // with respect to `x`, we apply L'Hôpital's rule. However, even after
        // doing this, the derivative is still zero (since p2 is also 0). So we
        // apply L'Hôpital's rule one more time until finally, we find the
        // correct derivative at 0.
        let coeffs = BezierCoefficients::new([(0.0, 0.0), (0.0, 1.0)]);
        assert!(coeffs.sample_dydx(0.0f32).is_finite());
    }

    #[test]
    fn test_infinite_gradient() {
        let coeffs = BezierCoefficients::new([(1.0, 0.0), (0.0, 1.0)]);

        assert!(coeffs.sample_dydx(0.5f32).is_finite());
    }

    #[test]
    fn test_zero_gradient() {
        let coeffs = BezierCoefficients::new([(0.0, 1.0), (1.0, 0.0)]);

        assert_eq!((coeffs.sample_dydx(0.5f32) * 100.0).round() / 100.0, 0.0);
    }
}
