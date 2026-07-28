use std::ops::{Add, Sub};

use crate::{Scalar, inner_space::InnerSpace};

pub trait EuclideanSpace:
    Copy
    + Sub<Self, Output = <Self as EuclideanSpace>::Diff>
    + Add<<Self as EuclideanSpace>::Diff, Output = Self>
    + PartialEq
{
    type Scalar: Scalar;
    type Diff: InnerSpace<Scalar = Self::Scalar>;

    fn origin() -> Self;
    fn to_vec(self) -> Self::Diff;
    fn from_vec(v: Self::Diff) -> Self;

    #[inline]
    fn distance2(self, other: Self) -> Self::Scalar
    where
        Self: std::ops::Sub<Self, Output = Self::Diff>,
    {
        (self - other).magnitude_squared()
    }

    #[inline]
    fn lerp(self, other: Self, t: Self::Scalar) -> Self
    where
        Self: std::ops::Add<Self::Diff, Output = Self>,
        Self: std::ops::Sub<Self, Output = Self::Diff>,
    {
        self + (other - self) * t
    }
}
