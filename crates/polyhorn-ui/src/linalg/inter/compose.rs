use num_traits::Float;

use super::algebra::{Combine, Cross, Dot, Vector};
use super::LaplaceExpansion3D;
use crate::linalg::{Quaternion3D, Transform3D};

/// This represents the decomposition of a 3D transformation matrix into
/// three-component translation, scale and skew vectors, a four-component
/// perspective vector and a rotation quaternion.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Decomposition3D<T> {
    /// This is the three-component translation vector of this decomposition.
    pub translation: [T; 3],

    /// This is the three-component scale vector of this decomposition.
    pub scale: [T; 3],

    /// This is the three-component skew vector of this decomposition.
    pub skew: [T; 3],

    /// This is the four-component perspective vector of this decomposition.
    pub perspective: [T; 4],

    /// This is the rotation quaternion of this decomposition.
    pub quaternion: Quaternion3D<T>,
}

impl<T> Decomposition3D<T> {
    /// Returns a new transformation matrix decomposition with the given
    /// translation, scale, skew, perspective and rotation quaternion.
    pub fn new(
        translation: [T; 3],
        scale: [T; 3],
        skew: [T; 3],
        perspective: [T; 4],
        quaternion: Quaternion3D<T>,
    ) -> Decomposition3D<T> {
        Decomposition3D {
            translation,
            scale,
            skew,
            perspective,
            quaternion,
        }
    }

    /// Applies the given operation to each element in this decomposition and
    /// returns the result. The operation may return a value of different type
    /// than the decomposition's previous value type.
    pub fn map<F, O>(self, mut op: F) -> Decomposition3D<O>
    where
        F: FnMut(T) -> O,
    {
        let [tx, ty, tz] = self.translation;
        let [sx, sy, sz] = self.scale;
        let [skx, sky, skz] = self.skew;
        let [px, py, pz, pw] = self.perspective;

        Decomposition3D {
            translation: [op(tx), op(ty), op(tz)],
            scale: [op(sx), op(sy), op(sz)],
            skew: [op(skx), op(sky), op(skz)],
            perspective: [op(px), op(py), op(pz), op(pw)],
            quaternion: self.quaternion.map(op),
        }
    }

    /// Returns a new decomposition with references to all elements of the
    /// previous composition. This is particularly useful if `T` does not
    /// implement `Copy`.
    pub fn as_ref(&self) -> Decomposition3D<&T> {
        let [tx, ty, tz] = &self.translation;
        let [sx, sy, sz] = &self.scale;
        let [skx, sky, skz] = &self.skew;
        let [px, py, pz, pw] = &self.perspective;

        Decomposition3D {
            translation: [tx, ty, tz],
            scale: [sx, sy, sz],
            skew: [skx, sky, skz],
            perspective: [px, py, pz, pw],
            quaternion: self.quaternion.as_ref(),
        }
    }
}

impl<T> Decomposition3D<T>
where
    T: Default + Float,
{
    /// Decomposes a 3D transformation matrix into its three-component
    /// translation, scale and skew vectors, its four-component perspective
    /// vector and its rotation quaternion.
    pub fn decompose(mut matrix: Transform3D<T>) -> Option<Decomposition3D<T>> {
        let mut result = Decomposition3D::default();

        // We start by normalizing the matrix. This is done by dividing each
        // entry by the bottom-right entry (i.e. the bottom right entry must
        // become 1). Of course, this isn't possible if the bottom right entry
        // is zero.
        if matrix.columns[3][3].is_zero() {
            return None;
        }

        for i in 0..4 {
            for j in 0..4 {
                matrix.columns[i][j] = matrix.columns[i][j] / matrix.columns[3][3];
            }
        }

        // perspectiveMatrix is used to solve for perspective, but it also
        // provides an easy way to test for singularity of the upper 3x3
        // component.
        let mut perspective_matrix = matrix;

        for i in 0..3 {
            perspective_matrix.columns[i][3] = T::zero();
        }

        perspective_matrix.columns[3][3] = T::one();

        let perspective_matrix = LaplaceExpansion3D::new(&perspective_matrix);

        if perspective_matrix.determinant().is_zero() {
            return None;
        }

        // First, isolate perspective.
        if !matrix.columns[0][3].is_zero()
            || !matrix.columns[1][3].is_zero()
            || !matrix.columns[2][3].is_zero()
        {
            unimplemented!(
                "Decomposing transformation matrices with a perspective is not yet implemented."
            )
        } else {
            // No perspective.
            result.perspective = [T::zero(), T::zero(), T::zero(), T::one()];
        }

        // Next, take care of translation.
        for i in 0..3 {
            result.translation[i] = matrix.columns[3][i];
        }

        // Now, get scale and shear.
        let mut row = [
            <[T; 3]>::default(),
            <[T; 3]>::default(),
            <[T; 3]>::default(),
        ];

        for i in 0..3 {
            row[i][0] = matrix.columns[i][0];
            row[i][1] = matrix.columns[i][1];
            row[i][2] = matrix.columns[i][2];
        }

        // Compute X scale factor and normalize first row.
        result.scale[0] = row[0].length();
        row[0] = row[0].normalize();

        // Compute XY shear factor and make 2nd row orthogonal to 1st.
        result.skew[0] = row[0].dot(&row[1]);
        row[1] = row[1].combine(T::one(), &row[0], -result.skew[0]);

        // Now, compute Y scale and normalize 2nd row.
        result.scale[1] = row[1].length();
        row[1] = row[1].normalize();
        result.skew[0] = result.skew[0] / result.scale[1];

        // Compute XZ and YZ shears, orthogonalize 3rd row.
        result.skew[1] = row[0].dot(&row[2]);
        row[2] = row[2].combine(T::one(), &row[0], -result.skew[1]);
        result.skew[2] = row[1].dot(&row[2]);
        row[2] = row[2].combine(T::one(), &row[1], -result.skew[2]);

        // Next, get Z scale and normalize 3rd row.
        result.scale[2] = row[2].length();
        row[2] = row[2].normalize();
        result.skew[1] = result.skew[1] / result.scale[2];
        result.skew[2] = result.skew[2] / result.scale[2];

        // At this point, the matrix (in rows) is orthonormal. Check for a
        // coordinate system flip. If the determinant is -1, then negate the
        // matrix and the scaling factors.
        let pdum3 = row[1].cross(&row[2]);

        if row[0].dot(&pdum3) < T::zero() {
            for i in 0..3 {
                result.scale[i] = result.scale[i].neg();
                row[i][0] = row[i][0].neg();
                row[i][1] = row[i][1].neg();
                row[i][2] = row[i][2].neg();
            }
        }

        // Now, get the rotations out.
        let half = T::from(0.5).unwrap();
        result.quaternion.x = half
            * (T::one() + row[0][0] - row[1][1] - row[2][2])
                .max(T::zero())
                .sqrt();
        result.quaternion.y = half
            * (T::one() - row[0][0] + row[1][1] - row[2][2])
                .max(T::zero())
                .sqrt();
        result.quaternion.z = half
            * (T::one() - row[0][0] - row[1][1] + row[2][2])
                .max(T::zero())
                .sqrt();
        result.quaternion.w = half
            * (T::one() + row[0][0] + row[1][1] + row[2][2])
                .max(T::zero())
                .sqrt();

        if row[2][1] > row[1][2] {
            result.quaternion.x = result.quaternion.x.neg();
        }

        if row[0][2] > row[2][0] {
            result.quaternion.y = result.quaternion.y.neg();
        }

        if row[1][0] > row[0][1] {
            result.quaternion.z = result.quaternion.z.neg();
        }

        Some(result)
    }

    /// This function interpolates between this decomposition and the other
    /// given decomposition with the given progress value. If the progress value
    /// is zero, it will return this decomposition. If the progress value is
    /// one, it will return the other decomposition.
    pub fn interpolate(&self, other: &Self, progress: T) -> Self {
        let mut result = *self;

        let alpha = T::one() - progress;
        let beta = progress;

        result.translation = result.translation.combine(alpha, &other.translation, beta);
        result.scale = result.scale.combine(alpha, &other.scale, beta);
        result.skew = result.skew.combine(alpha, &other.skew, beta);
        result.perspective = result.perspective.combine(alpha, &other.perspective, beta);
        result.quaternion = self.quaternion.mix(alpha, other.quaternion);

        result
    }

    /// Reconstructs the 3D transformation matrix from this decomposition.
    pub fn recompose(&self) -> Transform3D<T> {
        let mut result = Transform3D::identity();

        // Apply perspective.
        for i in 0..4 {
            result.columns[i][3] = self.perspective[i];
        }

        // Apply translation.
        for i in 0..4 {
            for j in 0..3 {
                result.columns[3][i] =
                    result.columns[3][i] + self.translation[j] * result.columns[j][i]
            }
        }

        // Apply rotation.
        result = result.rotate(self.quaternion);

        if !self.skew[2].is_zero() {
            let mut temp = Transform3D::identity();
            temp.columns[2][1] = self.skew[2];
            result = result.concat(temp);
        }

        if !self.skew[1].is_zero() {
            let mut temp = Transform3D::identity();
            temp.columns[2][0] = self.skew[1];
            result = result.concat(temp);
        }

        if !self.skew[0].is_zero() {
            let mut temp = Transform3D::identity();
            temp.columns[1][0] = self.skew[0];
            result = result.concat(temp);
        }

        // Apply scale.
        for i in 0..3 {
            for j in 0..4 {
                result.columns[i][j] = result.columns[i][j] * self.scale[i];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn interpolate<T>(a: Transform3D<T>, b: Transform3D<T>, t: T) -> Transform3D<T>
    where
        T: Default + Float,
    {
        let ad = Decomposition3D::decompose(a).unwrap();
        let bd = Decomposition3D::decompose(b).unwrap();
        let cd = ad.interpolate(&bd, t);
        cd.recompose()
    }

    #[test]
    fn test_halfway_rotation() {
        assert_eq!(
            interpolate(
                Transform3D::identity(),
                Transform3D::with_rotation(Quaternion3D::with_angle(
                    90.0 / 180.0 * PI,
                    0.0,
                    0.0,
                    1.0
                )),
                0.0
            ),
            Transform3D::with_rotation(Quaternion3D::with_angle(0.0 / 180.0 * PI, 0.0, 0.0, 1.0))
        );

        assert_eq!(
            interpolate(
                Transform3D::identity(),
                Transform3D::with_rotation(Quaternion3D::with_angle(
                    90.0 / 180.0 * PI,
                    0.0,
                    0.0,
                    1.0
                )),
                1.0
            ),
            Transform3D::with_rotation(Quaternion3D::with_angle(90.0 / 180.0 * PI, 0.0, 0.0, 1.0))
        );
    }

    #[test]
    fn test_halfway_rotation_nonunit() {
        assert_eq!(
            interpolate(
                Transform3D::identity(),
                Transform3D::with_rotation(Quaternion3D::with_angle(
                    90.0 / 180.0 * PI,
                    0.5,
                    0.3,
                    0.8
                )),
                0.5
            ),
            Transform3D::with_rotation(Quaternion3D::with_angle(45.0 / 180.0 * PI, 0.5, 0.3, 0.8))
        );
    }

    #[test]
    fn test_halfway_translation() {
        assert_eq!(
            interpolate(
                Transform3D::identity(),
                Transform3D::with_translation(160.0, 20.0, 10.0),
                0.5
            ),
            Transform3D::with_translation(80.0, 10.0, 5.0)
        );

        assert_eq!(
            interpolate(
                Transform3D::with_translation(160.0, 20.0, 10.0),
                Transform3D::identity(),
                0.5
            ),
            Transform3D::with_translation(80.0, 10.0, 5.0)
        );

        assert_eq!(
            interpolate(
                Transform3D::identity(),
                Transform3D::with_translation(-160.0, -20.0, 10.0),
                0.5
            ),
            Transform3D::with_translation(-80.0, -10.0, 5.0)
        );

        assert_eq!(
            interpolate(
                Transform3D::with_translation(-160.0, -20.0, 10.0),
                Transform3D::identity(),
                0.5
            ),
            Transform3D::with_translation(-80.0, -10.0, 5.0)
        );
    }

    #[test]
    fn test_decomposition_1() {
        let transform = Transform3D::with_translation(160.0, 20.0, 10.0)
            .rotate(Quaternion3D::with_angle(90.0 / 180.0 * PI, 0.0, 0.0, 1.0));

        let decomposition = Decomposition3D::decompose(transform).unwrap();

        let distance = decomposition.quaternion.subtract(&Quaternion3D::with_angle(
            90.0 / 180.0 * PI,
            0.0,
            0.0,
            1.0,
        ));

        assert!(distance.dot(&distance) <= 0.001);
    }

    #[test]
    fn test_decomposition_2() {
        let transform = Transform3D::with_translation(160.0, 20.0, 10.0)
            .rotate(Quaternion3D::with_angle(90.0 / 180.0 * PI, 0.0, 0.0, 1.0));

        let decomposition = Decomposition3D::decompose(transform).unwrap();

        assert_eq!(decomposition.translation, [160.0, 20.0, 10.0]);

        let distance = decomposition.quaternion.subtract(&Quaternion3D::with_angle(
            90.0 / 180.0 * PI,
            0.0,
            0.0,
            1.0,
        ));

        assert!(distance.dot(&distance) <= 0.001);
    }

    #[test]
    fn test_interpolation() {
        let start = Transform3D::identity();
        let end = Transform3D::with_translation(160.0, 20.0, 10.0)
            .rotate(Quaternion3D::with_angle(90.0 / 180.0 * PI, 0.0, 0.0, 1.0));

        let start = Decomposition3D::decompose(start).unwrap();
        let end = Decomposition3D::decompose(end).unwrap();

        let mid = start.interpolate(&end, 0.5);

        assert_eq!(mid.translation, [80.0, 10.0, 5.0]);

        let distance =
            mid.quaternion
                .subtract(&Quaternion3D::with_angle(45.0 / 180.0 * PI, 0.0, 0.0, 1.0));

        assert!(distance.dot(&distance) <= 0.001);
    }

    #[test]
    fn test_complex() {
        assert_eq!(
            interpolate(
                Transform3D::identity(),
                Transform3D::with_translation(160.0, 20.0, 10.0).rotate(Quaternion3D::with_angle(
                    90.0 / 180.0 * PI,
                    0.0,
                    0.0,
                    1.0
                )),
                0.5
            ),
            Transform3D::with_translation(80.0, 10.0, 5.0).rotate(Quaternion3D::with_angle(
                45.0 / 180.0 * PI,
                0.0,
                0.0,
                1.0
            ))
        );
    }
}
