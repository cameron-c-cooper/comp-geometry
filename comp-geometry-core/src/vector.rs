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

use crate::Storage;
pub use crate::{
    Scalar, matrix,
    matrix::{HMatrix, SMatrix},
};

pub type SVector<T, const N: usize> = SMatrix<T, N, 1, N>;
pub type HVector<T, const N: usize> = HMatrix<T, N, 1>;

pub trait VectorSpace: Sized {
    type Scalar: Scalar;
    fn dimension(&self) -> usize;
    fn dot(&self, rhs: &Self) -> Self::Scalar;
    fn norm_squared(&self) -> Self::Scalar {
        self.dot(self)
    }
}

// impl<T: Scalar> VectorSpace for Vector2<T> {
//     type Scalar = T;
//     #[inline]
//     fn dimension(&self) -> usize {
//         2
//     }
//     #[inline]
//     fn dot(&self, rhs: &Self) -> T {
//         self.x * rhs.x + self.y * rhs.y
//     }
// }

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
    ($name:ident, $dim:expr, { $($field:ident),+ $(,)? }) => {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name<T> {
            $(pub $field: T),+
        }
        impl<T> $name<T> {
            pub const DIM: usize = $dim;
            #[inline]
            pub const fn new($($field: T),+) -> Self {
                Self { $($field),+ }
            }
            #[inline]
            pub fn as_array(&self) -> &[T; $dim] {
                unsafe { &*(self as *const Self as *const [T; $dim]) }
            }
            #[inline]
            pub fn as_array_mut(&mut self) -> &mut [T; $dim] {
                unsafe { &mut *(self as *mut Self as *mut [T; $dim]) }
            }
        }
        impl<T> std::ops::Index<usize> for $name<T> {
            type Output = T;
            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                &self.as_array()[index]
            }
        }
        impl<T> std::ops::IndexMut<usize> for $name<T> {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.as_array_mut()[index]
            }
        }
        impl<T> From<[T; $dim]> for $name<T> where T: Copy {
            #[inline]
            fn from(arr: [T; $dim]) -> Self {
                // SAFETY: repr(C) forces layout to be accurate
                unsafe { std::ptr::read(arr.as_ptr() as *const Self) }
            }
        }
        impl<T> From<$name<T>> for [T; $dim] where T: Copy {
            #[inline]
            fn from(vec: $name<T>) -> Self {
                *vec.as_array()
            }
        }
        impl<T: Scalar, S: Storage<T>> From<$crate::matrix::Matrix<T, $dim, 1, S>>
        for $name<T> {
            #[inline]
            fn from(mat: $crate::matrix::Matrix<T, $dim, 1, S>) -> Self {
                let slice = mat.storage.as_slice();
                debug_assert_eq!(slice.len(), $dim);
                unsafe { std::ptr::read(slice.as_ptr() as *const Self) }
            }
        }
        impl<T: Scalar> From<$name<T>>
        for $crate::matrix::SMatrix<T, $dim, 1, $dim> {
            #[inline]
            fn from(vec: $name<T>) -> Self {
                $crate::matrix::Matrix {
                    storage: $crate::StackStorage {
                        data: *vec.as_array(),
                    },
                    _marker: std::marker::PhantomData
                }
            }
        }
        impl<T: Scalar> From<$name<T>>
        for $crate::matrix::HMatrix<T, $dim, 1> {
            #[inline]
            fn from(vec: $name<T>) -> Self {
                $crate::matrix::Matrix {
                    storage: $crate::HeapStorage {
                        data: vec.as_array().to_vec(),
                    },
                    _marker: std::marker::PhantomData
                }
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

        impl<T: $crate::Scalar> VectorSpace for $name<T> {
            type Scalar = T;
            #[inline]
            fn dimension(&self) -> usize {
                count_fields!($($field),+)
            }
            #[inline]
            fn dot(&self, rhs: &Self) -> T {
                let mut acc = T::zero();
                $( acc += self.$field * rhs.$field; )+
                acc
            }
        }
    };
}

macro_rules! count_fields {
    // Converts a list of fields into an array of 1s and takes its length at compile time
    ($($field:ident),*) => {
        <[()]>::len(&[$(count_fields!(@substitute $field)),*])
    };
    (@substitute $field:ident) => { () };
}

impl_vector!(Vector2, 2, { x, y });
impl_vector!(Vector3, 3, { x, y, z });
impl_vector!(Vector4, 4, { x, y, z, w });
impl_vector!(Vector5, 5, { x, y, z, u, v });
impl_vector!(Vector6, 6, { x, y, z, rx, ry, rz });
impl_vector!(Vector7, 7, { x, y, z, rx, ry, rz, warp });

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(vec2.dot(&vec1), 4);
    }

    #[test]
    fn matrix_from_vec2() {
        let vec1 = Vector2::new(0, 0);
        // TODO: See if I can simplify this
        let mat: SMatrix<_, _, _, 2> = SMatrix::from(vec1);
        let golden_mat = matrix![[0], [0]];
        assert_eq!(mat, golden_mat);
    }

    #[test]
    fn vec_from_matrix2() {
        let mat = matrix![[0], [0]];
        let vec1 = Vector2::from(mat);
        let golden_vec = Vector2::new(0, 0);
        assert_eq!(vec1, golden_vec);
    }

    #[test]
    fn matrix_from_vec3() {
        let vec1 = Vector3::new(0, 0, 0);
        let mat: SMatrix<_, _, _, 3> = SMatrix::from(vec1);
        let golden_mat = matrix![[0], [0], [0]];
        assert_eq!(mat, golden_mat);
    }

    #[test]
    fn vec_from_matrix3() {
        let mat = matrix![[0], [0], [0]];
        let vec1 = Vector3::from(mat);
        let golden_vec = Vector3::new(0, 0, 0);
        assert_eq!(vec1, golden_vec);
    }

    #[test]
    fn matrix_from_vec4() {
        let vec1 = Vector4::new(0, 0, 0, 0);
        let mat: SMatrix<_, _, _, 4> = SMatrix::from(vec1);
        let golden_mat = matrix![[0], [0], [0], [0]];
        assert_eq!(mat, golden_mat);
    }

    #[test]
    fn vec_from_matrix4() {
        let mat = matrix![[0], [0], [0], [0]];
        let vec1 = Vector4::from(mat);
        let golden_vec = Vector4::new(0, 0, 0, 0);
        assert_eq!(vec1, golden_vec);
    }

    #[test]
    fn matrix_from_vec5() {
        let vec1 = Vector5::new(0, 0, 0, 0, 0);
        let mat: SMatrix<_, _, _, 5> = SMatrix::from(vec1);
        let golden_mat = matrix![[0], [0], [0], [0], [0]];
        assert_eq!(mat, golden_mat);
    }

    #[test]
    fn vec_from_matrix5() {
        let mat = matrix![[0], [0], [0], [0], [0]];
        let vec1 = Vector5::from(mat);
        let golden_vec = Vector5::new(0, 0, 0, 0, 0);
        assert_eq!(vec1, golden_vec);
    }

    #[test]
    fn matrix_from_vec6() {
        let vec1 = Vector6::new(0, 0, 0, 0, 0, 0);
        let mat: SMatrix<_, _, _, 6> = SMatrix::from(vec1);
        let golden_mat = matrix![[0], [0], [0], [0], [0], [0]];
        assert_eq!(mat, golden_mat);
    }

    #[test]
    fn vec_from_matrix6() {
        let mat = matrix![[0], [0], [0], [0], [0], [0]];
        let vec1 = Vector6::from(mat);
        let golden_vec = Vector6::new(0, 0, 0, 0, 0, 0);
        assert_eq!(vec1, golden_vec);
    }

    #[test]
    fn matrix_from_vec7() {
        let vec1 = Vector7::new(0, 0, 0, 0, 0, 0, 0);
        let mat: SMatrix<_, _, _, 7> = SMatrix::from(vec1);
        let golden_mat = matrix![[0], [0], [0], [0], [0], [0], [0]];
        assert_eq!(mat, golden_mat);
    }

    #[test]
    fn vec_from_matrix7() {
        let mat = matrix![[0], [0], [0], [0], [0], [0], [0]];
        let vec1 = Vector7::from(mat);
        let golden_vec = Vector7::new(0, 0, 0, 0, 0, 0, 0);
        assert_eq!(vec1, golden_vec);
    }
}
