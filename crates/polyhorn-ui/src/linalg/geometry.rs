use num_traits::Float;
use std::ops::{Index, IndexMut};

/// A point in a 3D space.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Point3D<T> {
    /// The x-coordinate of this point.
    pub x: T,

    /// The y-coordinate of this point.
    pub y: T,

    /// The z-coordinate of this point.
    pub z: T,
}

impl<T> Point3D<T> {
    /// Returns a new point in a 3D space with the given coordinates.
    pub fn new(x: T, y: T, z: T) -> Point3D<T> {
        Point3D { x, y, z }
    }
}

impl<T> Point3D<T>
where
    T: Float,
{
    /// Returns the L2 norm of this vector (note to MLs: incl. the square root).
    pub fn l2_norm(self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns a normalized vector by dividing each component by the L2 norm
    /// of this vector.
    pub fn normalize(self) -> Point3D<T> {
        let norm = self.l2_norm();

        Point3D::new(self.x / norm, self.y / norm, self.z / norm)
    }
}

/// A 3D rotation quaternion.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Quaternion3D<T> {
    /// The x-element of this quaternion.
    pub x: T,

    /// The y-element of this quaternion.
    pub y: T,

    /// The z-element of this quaternion.
    pub z: T,

    /// The w-element of this quaternion.
    pub w: T,
}

impl<T> Quaternion3D<T> {
    /// Returns a new quaternion with the given elements.
    pub fn new(x: T, y: T, z: T, w: T) -> Quaternion3D<T> {
        Quaternion3D { x, y, z, w }
    }

    /// Applies the given transformation to each element of the given quaternion
    /// and returns the result.
    pub fn map<F, O>(self, mut op: F) -> Quaternion3D<O>
    where
        F: FnMut(T) -> O,
    {
        Quaternion3D {
            x: op(self.x),
            y: op(self.y),
            z: op(self.z),
            w: op(self.w),
        }
    }

    /// Returns a new quaternion with references to each of the original
    /// elements. This is particularly useful when `T` does not implement
    /// `Copy`.
    pub fn as_ref(&self) -> Quaternion3D<&T> {
        Quaternion3D {
            x: &self.x,
            y: &self.y,
            z: &self.z,
            w: &self.w,
        }
    }
}

impl<T> Quaternion3D<T>
where
    T: Float,
{
    /// Converts the rotation with the given angle radians around a vector with
    /// the given coordinates, into a quaternion with the corresponding
    /// elements.
    pub fn with_angle(angle: T, rx: T, ry: T, rz: T) -> Quaternion3D<T> {
        let r = Point3D::new(rx, ry, rz);
        let u = r.normalize();
        let angle = angle * r.l2_norm();

        let one = T::one();
        let two = one + one;

        let x = (angle / two).sin() * u.x;
        let y = (angle / two).sin() * u.y;
        let z = (angle / two).sin() * u.z;
        let w = (angle / two).cos();

        Quaternion3D { x, y, z, w }
    }

    /// Computes the inner product of this quaternion and the given quaternion.
    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Interpolates between the given quaternions with the given weight. If the
    /// weight is one, `self` is returned. If the weight is zero, `other` is
    /// returned.
    pub fn mix(self, weight: T, other: Quaternion3D<T>) -> Quaternion3D<T> {
        let mut product = self.dot(&other);
        product = product.min(T::one());
        product = product.max(T::one().neg());

        if product.abs().is_one() {
            return self;
        }

        let theta = product.acos();
        let w = ((T::one() - weight) * theta).sin() * (T::one() - product * product).sqrt().recip();

        let mut result = self;

        for i in 0..4 {
            let a = self[i] * (((T::one() - weight) * theta).cos() - product * w);
            let b = other[i] * w;
            result[i] = a + b;
        }

        result
    }

    /// Returns the element-wise addition of this quaternion and the given
    /// quaternion.
    pub fn addition(&self, other: &Quaternion3D<T>) -> Quaternion3D<T> {
        Quaternion3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    /// Returns the element-wise subtraction of this quaternion and the given
    /// quaternion.
    pub fn subtract(&self, other: &Quaternion3D<T>) -> Quaternion3D<T> {
        Quaternion3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T> Index<usize> for Quaternion3D<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 4);

        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => unreachable!(),
        }
    }
}

impl<T> IndexMut<usize> for Quaternion3D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 4);

        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => unreachable!(),
        }
    }
}
