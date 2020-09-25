use num_traits::Float;

use crate::linalg::Transform3D;

/// Container that memoizes the results of laplace expansion that is used to
/// compute the determinant, adjugate and (as a result) the inverse of a 3D
/// transformation matrix.
#[derive(Copy, Clone, Debug)]
pub struct LaplaceExpansion3D<'a, T>
where
    T: Float,
{
    /// Reference to the original transformation matrices that we will need to
    /// compute the adjugate matrix.
    pub transform: &'a Transform3D<T>,

    /// Coefficients of the _blue_ areas.
    pub blue: [T; 6],

    /// Coefficients of the _red_ areas.
    pub red: [T; 6],
}

impl<'a, T> LaplaceExpansion3D<'a, T>
where
    T: Float,
{
    /// Returns a new laplace expansion of the given transformation matrix.
    pub fn new(transform: &Transform3D<T>) -> LaplaceExpansion3D<T> {
        let [[a00, a10, a20, a30], [a01, a11, a21, a31], [a02, a12, a22, a32], [a03, a13, a23, a33]] =
            transform.columns;

        let s0 = a00 * a11 - a01 * a10;
        let c0 = a20 * a31 - a21 * a30;

        let s1 = a00 * a12 - a02 * a10;
        let c1 = a20 * a32 - a22 * a30;

        let s2 = a00 * a13 - a03 * a10;
        let c2 = a20 * a33 - a23 * a30;

        let s3 = a01 * a12 - a02 * a11;
        let c3 = a21 * a32 - a22 * a31;

        let s4 = a01 * a13 - a03 * a11;
        let c4 = a21 * a33 - a23 * a31;

        let s5 = a02 * a13 - a03 * a12;
        let c5 = a22 * a33 - a23 * a32;

        LaplaceExpansion3D {
            transform: &transform,
            blue: [s0, s1, s2, s3, s4, s5],
            red: [c0, c1, c2, c3, c4, c5],
        }
    }

    /// Returns the determinant of this transformation matrix.
    pub fn determinant(&self) -> T {
        let [s0, s1, s2, s3, s4, s5] = self.blue;
        let [c0, c1, c2, c3, c4, c5] = self.red;

        s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0
    }

    /// Returns the adjugate of this transformation matrix.
    pub fn adjugate(&self) -> Transform3D<T> {
        let [s0, s1, s2, s3, s4, s5] = self.blue;
        let [c0, c1, c2, c3, c4, c5] = self.red;

        let [[a00, a10, a20, a30], [a01, a11, a21, a31], [a02, a12, a22, a32], [a03, a13, a23, a33]] =
            self.transform.columns;

        Transform3D {
            columns: [
                [
                    a11 * c5 - a12 * c4 - a13 * c3,
                    -a10 * c5 + a12 * c2 - a13 * c1,
                    a10 * c4 - a11 * c2 + a13 * c0,
                    -a10 * c3 - a11 * c1 - a12 * c0,
                ],
                [
                    -a01 * c5 + a02 * c4 - a03 * c3,
                    a00 * c5 - a02 * c2 + a03 * c1,
                    -a00 * c4 + a01 * c2 - a03 * c0,
                    a00 * c3 - a01 * c1 + a02 * c0,
                ],
                [
                    a31 * s5 - a32 * s4 + a33 * s3,
                    -a30 * s5 + a32 * s2 - a33 * s1,
                    a30 * s4 - a31 * s2 + a33 * s0,
                    -a30 * s3 + a31 * s1 - a32 * s0,
                ],
                [
                    -a21 * s5 + a22 * s4 - a23 * s3,
                    a20 * s5 - a22 * s2 + a23 * s1,
                    -a20 * s4 + a21 * s2 - a23 * s0,
                    a20 * s3 - a21 * s1 + a22 * s0,
                ],
            ],
        }
    }

    /// Returns the inverse of this transformation matrix (if it exists) using
    /// the determinant and adjugate. If the determinant is zero, this function
    /// returns `None`.
    pub fn inverse(&self) -> Option<Transform3D<T>> {
        match self.determinant() {
            det if det.is_zero() => None,
            det => Some(self.adjugate().multiply_scalar(det.recip())),
        }
    }
}
