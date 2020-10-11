use num_traits::{Float, FloatConst};

use crate::geometry::{Dimension, Size};
use crate::linalg::{Quaternion3D, Transform3D};
use crate::physics::Angle;

/// Builder that can be used to efficiently create a transform consisting of
/// multiple individual operations.
///
/// The builder optimistically compresses the sequence of operations by
/// attempting to concatenate each operation to the previous operation, which
/// will succeed if the previous operation does not need to be resolved (i.e. is
/// non-relative). After compression, the builder can hold at most 8 transforms.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TransformBuilder<T>
where
    T: Float,
{
    transforms: [Transform<T>; 8],
    index: usize,
}

impl<T> TransformBuilder<T>
where
    T: Float,
{
    /// Returns a new builder that can be used to efficiently create a transform
    /// consisting of multiple individual operations.
    pub fn new() -> TransformBuilder<T>
    where
        T: Default,
    {
        Default::default()
    }

    /// Consumes the builder and returns a new builder after adding the given
    /// transform and recompression. Returns an error if the given transform
    /// cannot be concatenated to the last transform tracked by the builder and
    /// there are no slots remaining in the builder.
    pub fn push(
        mut self,
        transform: Transform<T>,
    ) -> Result<TransformBuilder<T>, TransformBuilder<T>> {
        match self.index {
            0 => self.transforms[0] = transform,
            n => match self.transforms[n - 1].concat(transform) {
                Some(transform) => {
                    self.transforms[n - 1] = transform;
                    return Ok(self);
                }
                None if n == self.transforms.len() => return Err(self),
                None => self.transforms[n] = transform,
            },
        }

        self.index += 1;

        Ok(self)
    }

    /// Returns the number of transforms currently tracked by the builder.
    pub fn len(&self) -> usize {
        self.index
    }

    /// Consumes the builder and returns all of its transforms.
    pub fn into_transforms(self) -> [Transform<T>; 8] {
        self.transforms
    }
}

/// Decomposition of a CSS transform into a constant 3D transform and a relative
/// 2D translation (i.e. CSS percentage).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform<T>
where
    T: Float,
{
    /// This is the underlying, layout-independent 3D affine transformation
    /// matrix.
    pub matrix: Transform3D<T>,

    /// This is the underlying, layout-dependent relative translation (measured
    /// in percentage points). This translation should be applied after the
    /// layout-independent 3D affine transformation matrix.
    pub relative_translation: (T, T),
}

impl<T> Transform<T>
where
    T: Float,
{
    /// This function creates a new transform with the given coefficients.
    pub fn with_transform(matrix: Transform3D<T>) -> Transform<T> {
        Transform {
            matrix,
            ..Default::default()
        }
    }

    /// This function creates a new homogeneous transform from the given
    /// coefficients.
    pub fn with_matrix(matrix: [T; 6]) -> Transform<T> {
        let [a, b, c, d, tx, ty] = matrix;

        let mut transform = Transform::default();
        transform.matrix.columns[0][0] = a;
        transform.matrix.columns[0][1] = b;
        transform.matrix.columns[1][0] = c;
        transform.matrix.columns[1][1] = d;
        transform.matrix.columns[3][0] = tx;
        transform.matrix.columns[3][1] = ty;

        transform
    }

    /// This function creates a new homogeneous transform from the given
    /// coefficients.
    pub fn with_matrix3d(matrix: [T; 16]) -> Transform<T> {
        let [m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44] =
            matrix;

        let mut transform = Transform::default();
        transform.matrix.columns[0] = [m11, m12, m13, m14];
        transform.matrix.columns[1] = [m21, m22, m23, m24];
        transform.matrix.columns[2] = [m31, m32, m33, m34];
        transform.matrix.columns[3] = [m41, m42, m43, m44];

        transform
    }

    /// This function creates a new perspective transform with the given
    /// parameters.
    pub fn with_perspective(d: T) -> Transform<T> {
        Transform {
            matrix: Transform3D::with_perspective(d),
            ..Default::default()
        }
    }

    /// This function creates a new rotation transform with the given
    /// parameters.
    pub fn with_rotation(rx: T, ry: T, rz: T, angle: Angle<T>) -> Transform<T>
    where
        T: FloatConst,
    {
        let q = Quaternion3D::with_angle(angle.to_radians(), -rx, -ry, rz);
        Transform {
            matrix: Transform3D::with_rotation(q),
            ..Default::default()
        }
    }

    /// This function creates a new scale transform with the given parameters.
    pub fn with_scale(sx: T, sy: T, sz: T) -> Transform<T> {
        Transform {
            matrix: Transform3D::with_scale(sx, sy, sz),
            ..Default::default()
        }
    }

    /// This function creates a new horizontal skew transform with the given
    /// angle.
    pub fn with_skew_x(sx: Angle<T>) -> Transform<T> {
        Transform {
            matrix: Transform3D::with_skew_x(sx.to_radians()),
            ..Default::default()
        }
    }

    /// This function creates a new vertical skew transform with the given
    /// angle.
    pub fn with_skew_y(sy: Angle<T>) -> Transform<T> {
        Transform {
            matrix: Transform3D::with_skew_y(sy.to_radians()),
            ..Default::default()
        }
    }

    /// This function creates a new translation transform with the given
    /// parameters. If one of the horizontal or vertical dimensions is relative
    /// (i.e. a percentage), this transform will need to be resolved before it
    /// can be applied.
    pub fn with_translation(tx: Dimension<T>, ty: Dimension<T>, tz: T) -> Transform<T> {
        let (tx, rx) = match tx {
            Dimension::Percentage(rx) => (T::zero(), rx),
            Dimension::Points(tx) => (tx, T::zero()),
            _ => (T::zero(), T::zero()),
        };

        let (ty, ry) = match ty {
            Dimension::Percentage(ry) => (T::zero(), ry),
            Dimension::Points(ty) => (ty, T::zero()),
            _ => (T::zero(), T::zero()),
        };

        Transform {
            matrix: Transform3D::with_translation(tx, ty, tz),
            relative_translation: (rx, ry),
        }
    }

    /// This function returns a boolean that indicates if this transform is
    /// resolved, i.e. its relative translation is zero in both dimensions.
    pub fn is_resolved(&self) -> bool {
        let (tx, ty) = self.relative_translation;
        tx.is_zero() && ty.is_zero()
    }

    /// This function resolves a transform using the given size.
    pub fn resolve(&self, size: Size<T>) -> Transform3D<T> {
        let (rtx, rty) = self.relative_translation;

        self.matrix
            .translate(rtx * size.width, rty * size.height, T::zero())
    }

    /// This function attempts to compress this transform and the given
    /// transform into a single transform, which will only work if the
    /// original transform is resolved (i.e. not relative to the unknown
    /// container size).
    pub fn concat(&self, other: Transform<T>) -> Option<Transform<T>> {
        match self.is_resolved() {
            true => Some(Transform {
                matrix: self.matrix.concat(other.matrix),
                relative_translation: other.relative_translation,
            }),
            false => None,
        }
    }

    /// This function folds the given transforms after resolving each with the
    /// given size.
    pub fn squash(transforms: [Transform<T>; 8], size: Size<T>) -> Transform3D<T> {
        transforms
            .iter()
            .fold(Transform3D::identity(), |current, transform| {
                current.concat(transform.resolve(size))
            })
    }
}

impl<T> Default for Transform<T>
where
    T: Float,
{
    fn default() -> Self {
        Transform {
            matrix: Default::default(),
            relative_translation: (T::zero(), T::zero()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Angle, Dimension, Quaternion3D, Size, Transform, Transform3D, TransformBuilder};

    #[test]
    fn test_transform() {
        assert_eq!(
            Transform::with_rotation(1.0, 0.0, 0.0, Angle::with_degrees(45.0)).is_resolved(),
            true
        );

        assert_eq!(Transform::with_scale(1.0, 2.0, 3.0).is_resolved(), true);

        assert_eq!(
            Transform::with_translation(Dimension::Points(10.0), Dimension::Points(20.0), 30.0)
                .is_resolved(),
            true
        );

        assert_eq!(
            Transform::with_translation(Dimension::Percentage(10.0), Dimension::Points(20.0), 30.0)
                .is_resolved(),
            false
        );

        assert_eq!(
            Transform::with_translation(Dimension::Points(10.0), Dimension::Percentage(20.0), 30.0)
                .is_resolved(),
            false
        );

        assert_eq!(
            Transform::with_translation(
                Dimension::Percentage(0.0),
                Dimension::Percentage(0.0),
                30.0
            )
            .is_resolved(),
            true
        );
    }

    #[test]
    fn test_transform_builder() {
        let builder = TransformBuilder::new()
            .push(Transform::with_scale(2.0, 3.0, 1.0))
            .unwrap()
            .push(Transform::with_translation(
                Dimension::Percentage(0.5),
                Dimension::Percentage(0.3),
                20.0,
            ))
            .unwrap()
            .push(Transform::with_rotation(
                1.0,
                0.0,
                0.0,
                Angle::with_degrees(45.0),
            ))
            .unwrap();

        assert_eq!(builder.len(), 2);

        let resolved = Transform::squash(builder.into_transforms(), Size::new(320.0, 480.0));

        assert_eq!(
            resolved,
            Transform3D::with_scale(2.0, 3.0, 1.0)
                .translate(160.0, 144.0, 20.0)
                .rotate(Quaternion3D::with_angle(
                    Angle::with_degrees(45.0).to_radians(),
                    -1.0,
                    0.0,
                    0.0
                ))
        );
    }
}
