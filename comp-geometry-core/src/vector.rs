// concrete spatial primitives, Vector2, 3, Point 2, 3
// |
// |
// Generic fixed vector
// SVector<T, const N: usize> (N x 1 matrix wrapper)
// indexable
// |
// |
// Dyn arbitrary vectory
// HVector<T, const N: usize>

pub use crate::{
    Scalar, matrix,
    matrix::{HMatrix, SMatrix},
};
use crate::{Storage, matrix::Matrix};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Scalar> Vector2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    #[inline]
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y + rhs.y
    }
}

impl<T: Scalar> Vector3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    #[inline]
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y + rhs.y + self.z + rhs.z
    }
}

pub type SVector<T, const N: usize> = SMatrix<T, N, 1, N>;
pub type HVector<T, const N: usize> = HMatrix<T, N, 1>;

impl<T: Copy + Scalar> From<Vector2<T>> for SVector<T, 2> {
    #[inline]
    fn from(v: Vector2<T>) -> Self {
        matrix![[v.x], [v.y]]
    }
}

impl<T: Copy + Scalar> From<SVector<T, 2>> for Vector2<T> {
    #[inline]
    fn from(value: SVector<T, 2>) -> Self {
        Vector2 {
            x: value[0][0],
            y: value[1][0],
        }
    }
}

pub trait VectorSpace: Sized {
    type Scalar: Scalar;
    fn dimension(&self) -> usize;
    fn dot(&self, rhs: &Self) -> Self::Scalar;
    fn norm_squared(&self) -> Self::Scalar {
        self.dot(self)
    }
}

impl<T: Scalar> VectorSpace for Vector2<T> {
    type Scalar = T;
    #[inline]
    fn dimension(&self) -> usize {
        2
    }
    #[inline]
    fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: Scalar> VectorSpace for Vector3<T> {
    type Scalar = T;
    #[inline]
    fn dimension(&self) -> usize {
        3
    }
    #[inline]
    fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: Scalar, const N: usize, S> VectorSpace for Matrix<T, N, 1, S>
where
    S: Storage<T>,
{
    type Scalar = T;
    #[inline]
    fn dimension(&self) -> usize {
        N
    }
    #[inline]
    fn dot(&self, rhs: &Self) -> Self::Scalar {
        let mut sum = T::zero();
        let lhs_s = self.storage.as_slice();
        let rhs_s = rhs.storage.as_slice();
        for i in 0..N {
            sum += lhs_s[i] * rhs_s[i];
        }
        sum
    }
}
