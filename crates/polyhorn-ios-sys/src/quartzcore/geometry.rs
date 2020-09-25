use crate::coregraphics::CGFloat;

/// The standard transform matrix used throughout Core Animation.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CATransform3D {
    /// The entry at position 1,1 in the matrix.
    pub m11: CGFloat,

    /// The entry at position 1,2 in the matrix.
    pub m12: CGFloat,

    /// The entry at position 1,3 in the matrix.
    pub m13: CGFloat,

    /// The entry at position 1,4 in the matrix.
    pub m14: CGFloat,

    /// The entry at position 21 in the matrix.
    pub m21: CGFloat,

    /// The entry at position 22 in the matrix.
    pub m22: CGFloat,

    /// The entry at position 23 in the matrix.
    pub m23: CGFloat,

    /// The entry at position 24 in the matrix.
    pub m24: CGFloat,

    /// The entry at position 3,1 in the matrix.
    pub m31: CGFloat,

    /// The entry at position 3,2 in the matrix.
    pub m32: CGFloat,

    /// The entry at position 3,3 in the matrix.
    pub m33: CGFloat,

    /// The entry at position 3,4 in the matrix.
    pub m34: CGFloat,

    /// The entry at position 4,1 in the matrix.
    pub m41: CGFloat,

    /// The entry at position 4,2 in the matrix.
    pub m42: CGFloat,

    /// The entry at position 4,3 in the matrix.
    pub m43: CGFloat,

    /// The entry at position 4,4 in the matrix.
    pub m44: CGFloat,
}

impl Default for CATransform3D {
    fn default() -> Self {
        CATransform3D {
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            m14: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m24: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,
            m34: 0.0,
            m41: 0.0,
            m42: 0.0,
            m43: 0.0,
            m44: 1.0,
        }
    }
}
