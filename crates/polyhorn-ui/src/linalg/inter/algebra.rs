use num_traits::Float;

pub trait Dot {
    type Output;

    fn dot(&self, other: &Self) -> Self::Output;
}

impl<T> Dot for [T; 3]
where
    T: Float,
{
    type Output = T;

    fn dot(&self, other: &Self) -> T {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }
}

impl<T> Dot for [T; 4]
where
    T: Float,
{
    type Output = T;

    fn dot(&self, other: &Self) -> T {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2] + self[3] * other[3]
    }
}

pub trait Cross {
    fn cross(&self, other: &Self) -> Self;
}

impl<T> Cross for [T; 3]
where
    T: Float,
{
    fn cross(&self, other: &Self) -> Self {
        [
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ]
    }
}

pub trait Vector<T>: Sized {
    fn length(&self) -> T;
    fn normalize(&self) -> Self;
}

impl<T> Vector<T> for [T; 3]
where
    T: Float,
{
    fn length(&self) -> T {
        self.dot(self).sqrt()
    }

    fn normalize(&self) -> Self {
        let length = self.length();

        [self[0] / length, self[1] / length, self[2] / length]
    }
}

pub trait Combine<T> {
    fn combine(&self, alpha: T, other: &Self, beta: T) -> Self;
}

impl<T> Combine<T> for [T; 3]
where
    T: Float,
{
    fn combine(&self, alpha: T, other: &Self, beta: T) -> Self {
        [
            self[0] * alpha + other[0] * beta,
            self[1] * alpha + other[1] * beta,
            self[2] * alpha + other[2] * beta,
        ]
    }
}

impl<T> Combine<T> for [T; 4]
where
    T: Float,
{
    fn combine(&self, alpha: T, other: &Self, beta: T) -> Self {
        [
            self[0] * alpha + other[0] * beta,
            self[1] * alpha + other[1] * beta,
            self[2] * alpha + other[2] * beta,
            self[3] * alpha + other[3] * beta,
        ]
    }
}
