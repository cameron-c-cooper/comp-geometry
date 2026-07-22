use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use crate::Scalar;

pub trait InnerSpace:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Mul<<Self as InnerSpace>::Scalar, Output = Self>
    + Div<<Self as InnerSpace>::Scalar, Output = Self>
    + AddAssign
    + SubAssign
{
    type Scalar: Scalar;

    fn dot(self, rhs: Self) -> Self::Scalar;
    fn magnitude_squared(self) -> Self::Scalar {
        self.dot(self)
    }
}
