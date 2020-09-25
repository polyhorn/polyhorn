use num_traits::Float;

use super::{Point3D, Quaternion3D};

/// Rank 4 row-major transformation matrix for 3D objects.
#[derive(Copy, Clone, Debug)]
pub struct Transform3D<T> {
    /// This field contains the entries of this matrix in column-major order.
    pub columns: [[T; 4]; 4],
}

impl<T> Transform3D<T> {
    /// Returns a new transformation matrix with the given entries in row-major
    /// order. This function does not perform any kind of validation on the
    /// given entries. It is the caller's responsibility to ensure that the
    /// given entries result in a valid transformation matrix.
    pub fn new(columns: [[T; 4]; 4]) -> Transform3D<T> {
        Transform3D { columns }
    }

    /// Applies the given transformation to each entry in the transformation
    /// matrix individually.
    pub fn map<F, O>(self, mut op: F) -> Transform3D<O>
    where
        F: FnMut(T) -> O,
    {
        let [[a00, a01, a02, a03], [a10, a11, a12, a13], [a20, a21, a22, a23], [a30, a31, a32, a33]] =
            self.columns;

        Transform3D {
            columns: [
                [op(a00), op(a01), op(a02), op(a03)],
                [op(a10), op(a11), op(a12), op(a13)],
                [op(a20), op(a21), op(a22), op(a23)],
                [op(a30), op(a31), op(a32), op(a33)],
            ],
        }
    }
}

impl<T> Transform3D<T>
where
    T: Float,
{
    /// Returns a new identity matrix, i.e. a matrix of zeros with ones on the
    /// diagonal.
    pub fn identity() -> Transform3D<T> {
        let i = T::one();
        let o = T::zero();

        Transform3D {
            columns: [[i, o, o, o], [o, i, o, o], [o, o, i, o], [o, o, o, i]],
        }
    }

    /// Returns a new transformation matrix for a perspective with the given
    /// depth.
    pub fn with_perspective(d: T) -> Transform3D<T> {
        let mut columns = Self::identity().columns;
        columns[2][3] = d.recip();
        Transform3D { columns }
    }

    /// Returns a new transformation matrix for a translation with the given
    /// offsets.
    pub fn with_translation(tx: T, ty: T, tz: T) -> Transform3D<T> {
        let mut columns = Self::identity().columns;
        columns[3][0] = tx;
        columns[3][1] = ty;
        columns[3][2] = tz;
        Transform3D { columns }
    }

    /// Returns a new transformation matrix for a scale with the given factors.
    pub fn with_scale(sx: T, sy: T, sz: T) -> Transform3D<T> {
        let mut columns = Self::identity().columns;
        columns[0][0] = sx;
        columns[1][1] = sy;
        columns[2][2] = sz;
        Transform3D { columns }
    }

    /// Returns a new transformation matrix for a horizontal skew with the
    /// given angle radians.
    pub fn with_skew_x(sx: T) -> Transform3D<T> {
        let mut columns = Self::identity().columns;
        columns[1][0] = sx.tan();
        Transform3D { columns }
    }

    /// Returns a new transformation matrix for a vertical skew with the given
    /// angle radians.
    pub fn with_skew_y(sy: T) -> Transform3D<T> {
        let mut columns = Self::identity().columns;
        columns[0][1] = sy.tan();
        Transform3D { columns }
    }

    /// Returns a new transformation matrix for a counter-clockwise rotation
    /// with the given angle radians around the given vector. The given vector
    /// does not have to be a unit vector. If it is not a unit vector, this
    /// function will normalize and and multiply the angle with the original
    /// norm.
    pub fn with_rotation(q: Quaternion3D<T>) -> Transform3D<T> {
        let Quaternion3D { x, y, z, w } = q;

        let one = T::one();
        let two = one + one;

        let mut columns = Self::identity().columns;
        columns[0][0] = one - two * (y * y + z * z);
        columns[0][1] = two * (x * y + z * w);
        columns[0][2] = two * (x * z - y * w);
        columns[1][0] = two * (x * y - z * w);
        columns[1][1] = one - two * (x * x + z * z);
        columns[1][2] = two * (y * z + x * w);
        columns[2][0] = two * (x * z + y * w);
        columns[2][1] = two * (y * z - x * w);
        columns[2][2] = one - two * (x * x + y * y);

        Transform3D { columns }
    }

    /// Translates the given transformation matrix with the given offsets by
    /// concatenating the given matrix to a new translation matrix.
    pub fn pre_translate(self, tx: T, ty: T, tz: T) -> Transform3D<T> {
        self.pre_multiply(Self::with_translation(tx, ty, tz))
    }

    /// Scales the given transformation matrix with the given factors by
    /// concatenating the given matrix to a new scale matrix.
    pub fn pre_scale(self, sx: T, sy: T, sz: T) -> Transform3D<T> {
        self.pre_multiply(Self::with_scale(sx, sy, sz))
    }

    /// Rotates the given transformation matrix with the given angle in radians
    /// around the given vector. The rotation is counter-clockwise and if the
    /// given vector is not unit, it will be normalized and the angle will be
    /// multiplied with the length of the original vector.
    pub fn pre_rotate(self, q: Quaternion3D<T>) -> Transform3D<T> {
        self.pre_multiply(Self::with_rotation(q))
    }

    /// Translates the given transformation matrix with the given offsets by
    /// concatenating the given matrix to a new translation matrix.
    pub fn translate(self, tx: T, ty: T, tz: T) -> Transform3D<T> {
        self.multiply(Self::with_translation(tx, ty, tz))
    }

    /// Scales the given transformation matrix with the given factors by
    /// concatenating the given matrix to a new scale matrix.
    pub fn scale(self, sx: T, sy: T, sz: T) -> Transform3D<T> {
        self.multiply(Self::with_scale(sx, sy, sz))
    }

    /// Rotates the given transformation matrix with the given angle in radians
    /// around the given vector. The rotation is counter-clockwise and if the
    /// given vector is not unit, it will be normalized and the angle will be
    /// multiplied with the length of the original vector.
    pub fn rotate(self, q: Quaternion3D<T>) -> Transform3D<T> {
        self.multiply(Self::with_rotation(q))
    }

    /// Concatenates the given transformation matrix to the current
    /// transformation matrix.
    pub fn multiply(self, other: Transform3D<T>) -> Transform3D<T> {
        macro_rules! dot {
            ($c:expr, $a:expr, $b:expr, $i:literal, $j:literal) => {
                $c[$i][$j] = $a[$i][0] * $b[0][$j]
                    + $a[$i][1] * $b[1][$j]
                    + $a[$i][2] * $b[2][$j]
                    + $a[$i][3] * $b[3][$j];
            };
        }

        let mut columns = Self::identity().columns;
        dot!(columns, self.columns, other.columns, 0, 0);
        dot!(columns, self.columns, other.columns, 0, 1);
        dot!(columns, self.columns, other.columns, 0, 2);
        dot!(columns, self.columns, other.columns, 0, 3);
        dot!(columns, self.columns, other.columns, 1, 0);
        dot!(columns, self.columns, other.columns, 1, 1);
        dot!(columns, self.columns, other.columns, 1, 2);
        dot!(columns, self.columns, other.columns, 1, 3);
        dot!(columns, self.columns, other.columns, 2, 0);
        dot!(columns, self.columns, other.columns, 2, 1);
        dot!(columns, self.columns, other.columns, 2, 2);
        dot!(columns, self.columns, other.columns, 2, 3);
        dot!(columns, self.columns, other.columns, 3, 0);
        dot!(columns, self.columns, other.columns, 3, 1);
        dot!(columns, self.columns, other.columns, 3, 2);
        dot!(columns, self.columns, other.columns, 3, 3);

        Transform3D { columns }
    }

    /// Short hand to multiply matrices in reverse order (which is called
    /// pre-multiplication in the graphics industry).
    pub fn pre_multiply(self, other: Transform3D<T>) -> Transform3D<T> {
        other.multiply(self)
    }

    /// Applies the given transformation matrix to a point.
    pub fn apply(self, point: Point3D<T>) -> Point3D<T> {
        let x = self.columns[0][0] * point.x
            + self.columns[1][0] * point.y
            + self.columns[2][0] * point.z
            + self.columns[3][0];

        let y = self.columns[0][1] * point.x
            + self.columns[1][1] * point.y
            + self.columns[2][1] * point.z
            + self.columns[3][1];

        let z = self.columns[0][2] * point.x
            + self.columns[1][2] * point.y
            + self.columns[2][2] * point.z
            + self.columns[3][2];

        let q = self.columns[0][3] * point.x
            + self.columns[1][3] * point.y
            + self.columns[2][3] * point.z
            + self.columns[3][3];

        Point3D {
            x: x / q,
            y: y / q,
            z: z / q,
        }
    }

    /// Multiplies every element of this transformation with the given scalar
    /// and returns the result.
    pub fn multiply_scalar(mut self, scalar: T) -> Transform3D<T> {
        for i in 0..4 {
            for j in 0..4 {
                self.columns[i][j] = self.columns[i][j] * scalar;
            }
        }

        self
    }

    fn subtract(self, other: Transform3D<T>) -> Transform3D<T> {
        let mut columns = self.columns;
        for i in 0..4 {
            for j in 0..4 {
                columns[i][j] = self.columns[i][j] - other.columns[i][j];
            }
        }
        Transform3D { columns }
    }

    fn l2_norm(self) -> T {
        (0..4).into_iter().fold(T::zero(), |acc, i| {
            acc + (0..4).into_iter().fold(T::zero(), |acc, j| {
                acc + self.columns[i][j] * self.columns[i][j]
            })
        })
    }
}

impl<T> Default for Transform3D<T>
where
    T: Float,
{
    fn default() -> Self {
        Transform3D::identity()
    }
}

impl<T> Eq for Transform3D<T> where T: Float {}

impl<T> PartialEq for Transform3D<T>
where
    T: Float,
{
    fn eq(&self, other: &Transform3D<T>) -> bool {
        self.subtract(*other).l2_norm() < T::from(1.0 / 1000.0).unwrap()
    }
}
