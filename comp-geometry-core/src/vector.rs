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

// impl<T: Scalar> Add for Vector2<T> {
//     type Output = Self;
//     #[inline]
//     fn add(self, rhs: Self) -> Self {
//         Self::new(self.x + rhs.x, self. y + rhs.y)
//     }
// }
//
// impl<T: Scalar> Sub for Vector2<T> {
//     type Output = Self;
//     #[inline]
//     fn sub(self, rhs: Self) -> Self {
//         Self::new(self.x - rhs.x, self. y - rhs.y)
//     }
// }
//
// impl<T: SignedScalar> Neg for Vector2<T> {
//     type Output = Self;
//     #[inline]
//     fn neg(self) -> Self::Output {
//         Self::new(-self.x, -self.y)
//     }
// }
//
// impl<T: Scalar> Mul<T> for Vector2<T> {
//
// }

macro_rules! impl_vector {
    ($name:ident { $($field:ident),+ $(,)? }) => {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name<T> {
            $(pub $field: T),+
        }
        impl<T> $name<T> {
            #[inline]
            pub const fn new($($field: T),+) -> Self {
                Self { $($field),+ }
            }
        }
        impl<T: $crate::Scalar> std::ops::Add for $name<T> {
            type Output = Self;
            #[inline]
            fn add(self, rhs: Self) -> Self {
                Self::new($(self.$field + rhs.$field),+)
            }
        }

        impl<T: $crate::Scalar> std::ops::Sub for $name<T> {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: Self) -> Self {
                Self::new($(self.$field - rhs.$field),+)
            }
        }

        impl<T: $crate::SignedScalar> std::ops::Neg for $name<T> {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self::Output {
                Self::new($(-self.$field),+)
            }
        }

        impl<T: $crate::Scalar> std::ops::Mul<T> for $name<T> {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: T) -> Self {
                Self::new($(self.$field * rhs),+)
            }
        }

        impl<T: $crate::Scalar> std::ops::Div<T> for $name<T> {
            type Output = Self;
            #[inline]
            fn div(self, rhs: T) -> Self {
                Self::new($(self.$field / rhs),+)
            }
        }

        impl<T: $crate::Scalar> std::ops::AddAssign for $name<T> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                $(self.$field += rhs.$field;)+
            }
        }

        impl<T: $crate::Scalar> std::ops::SubAssign for $name<T> {
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$field -= rhs.$field;)+
            }
        }

        impl<T: $crate::SignedScalar> $crate::inner_space::InnerSpace for $name<T> {
            type Scalar = T;
            #[inline]
            fn dot(self, rhs: Self) -> T {
                let mut acc = T::zero();
                $( acc += self.$field * rhs.$field; )+
                acc
            }
        }
    };
}

impl_vector!(Vector2 { x, y });
impl_vector!(Vector3 { x, y, z });

#[cfg(test)]
mod tests {
    use crate::inner_space::InnerSpace;
    use crate::vector::Vector2;

    #[test]
    fn vec_macro_impl() {
        let vec1 = Vector2::new(0, 1);
        let vec2 = Vector2::new(2, 4);
        let mut comb_vec = Vector2::new(0, 0);
        assert_eq!(vec1 + vec2, Vector2::new(2, 5));
        assert_eq!(vec1 - vec2, Vector2::new(-2, -3));
        assert_eq!(-vec1, Vector2::new(0, -1));
        assert_eq!(vec1 * 2, Vector2::new(0, 2));
        assert_eq!(vec2 / 2, Vector2::new(1, 2));
        comb_vec += vec1;
        assert_eq!(comb_vec, vec1);
        comb_vec -= vec1;
        assert_eq!(comb_vec, Vector2::new(0, 0));
        assert_eq!(vec2.dot(vec1), 4);
    }
}
