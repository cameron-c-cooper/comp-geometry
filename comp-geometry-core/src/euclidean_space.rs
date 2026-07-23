use std::ops::{Add, Sub};

use crate::{FloatVal, Scalar, inner_space::InnerSpace};

pub trait EuclideanSpace:
    Copy 
    + Sub<Self, Output = <Self as EuclideanSpace>::Vector>
    + Add<<Self as EuclideanSpace>::Vector, Output = Self>
    + PartialEq
{
    type Scalar: Scalar;
    type Vector: InnerSpace<Scalar = Self::Scalar>;
    fn origin() -> Self;
    fn from_vec(v: Self::Vector) -> Self;
    fn to_vec(self) -> Self::Vector;
    fn distance2(self, other: Self) -> Self::Scalar {
        (self - other).magnitude_squared()
    }
    fn distance(self, other: Self) -> Self::Scalar
    where
        Self::Scalar: FloatVal
    {
       self.distance2(other).sqrt() 
    }

    fn lerp(self, other: Self, t: Self::Scalar) -> Self
    where 
        Self::Scalar: Copy,
        Self::Vector: std::ops::Mul<Self::Scalar, Output = Self::Vector>,
    {
        self + (other - self) * t
    }
}
