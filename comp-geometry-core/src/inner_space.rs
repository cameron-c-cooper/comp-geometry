use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use crate::{RealScalar, Scalar};

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
    #[inline]
    fn magnitude_squared(self) -> Self::Scalar {
        self.dot(self)
    }
    #[inline]
    fn magnitude(self) -> Self::Scalar
    where
        Self::Scalar: RealScalar,
    {
        self.magnitude_squared().sqrt()
    }

    #[inline]
    fn normalize(self) -> Self
    where
        Self::Scalar: RealScalar,
    {
        let mag = self.magnitude();
        if mag == Self::Scalar::zero() {
            self
        } else {
            self / mag
        }
    }

    #[inline]
    fn project_on(self, other: Self) -> Self {
        other * (self.dot(other) / other.magnitude_squared())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::Vector3;
    #[test]
    fn magnitude() {
        let mag = Vector3::new(1.0, 2.0, 0.0).magnitude();
        let golden_val = 5.0f32.sqrt();
        assert!((mag - golden_val).abs() < f32::EPSILON);
    }
}
