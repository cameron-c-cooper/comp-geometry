#![feature(generic_const_exprs)]
// #![feature(macro_metavar_expr)]
#![allow(incomplete_features)]

use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub mod euclidean_space;
pub mod inner_space;
pub mod matrix;
pub mod vector;

pub trait Scalar:
    Copy
    + PartialOrd
    + Debug
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    // + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + 'static
{
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait SignedScalar: Scalar + Neg<Output = Self> {}

pub trait RealScalar: Scalar {
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn atan2(y: Self, x: Self) -> Self;
    fn abs(self) -> Self;
}

macro_rules! impl_scalar_float {
    ($($t:ty),*) => {
        $(
            impl Scalar for $t {
                #[inline]
                fn zero() -> Self { 0.0 }
                #[inline]
                fn one() -> Self { 1.0 }
                #[inline]
                fn is_zero(&self) -> bool { self.abs() < <$t>::EPSILON }
            }

            impl RealScalar for $t {
                #[inline] fn sqrt(self) -> Self { self.sqrt() }
                #[inline] fn sin(self) -> Self { self.sin() }
                #[inline] fn cos(self) -> Self { self.cos() }
                #[inline] fn atan2(y: Self, x: Self) -> Self { y.atan2(x) }
                #[inline] fn abs(self) -> Self { self.abs() }
            }

            impl SignedScalar for $t {}
        )*
    };
}

macro_rules! impl_scalar_sint {
    ($($t:ty),*) => {
        $(
            impl Scalar for $t {
                #[inline]
                fn zero() -> Self { 0 }
                #[inline]
                fn one() -> Self { 1 }
                #[inline]
                fn is_zero(&self) -> bool { *self == 0 }
            }

            impl SignedScalar for $t {}
        )*
    };
}

macro_rules! impl_scalar_uint {
    ($($t:ty),*) => {
        $(
            impl Scalar for $t {
                #[inline]
                fn zero() -> Self { 0 }
                #[inline]
                fn one() -> Self { 1 }
                #[inline]
                fn is_zero(&self) -> bool { *self == 0 }
            }
        )*
    };
}

// not gonna support uint
impl_scalar_float!(f32, f64);
impl_scalar_sint!(i32, i64, i128, isize);
impl_scalar_uint!(u32, u64, u128, usize);

pub trait Storage<T>: Index<usize, Output = T> + IndexMut<usize, Output = T> {
    type SameSize<const N: usize>: Storage<T>;
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];
    fn zeros(len: usize) -> Self
    where
        T: Scalar;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackStorage<T, const N: usize> {
    pub data: [T; N],
}

impl<T, const N: usize> Index<usize> for StackStorage<T, N> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for StackStorage<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Scalar, const N: usize> Storage<T> for StackStorage<T, N> {
    type SameSize<const M: usize> = StackStorage<T, M>;
    #[inline]
    fn as_slice(&self) -> &[T] {
        &self.data
    }
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
    #[inline]
    // dont need the len, type system encodes it
    fn zeros(_: usize) -> Self
    where
        T: Scalar,
    {
        Self {
            data: [T::zero(); N],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeapStorage<T> {
    pub data: Vec<T>,
}

impl<T> Index<usize> for HeapStorage<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for HeapStorage<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Scalar> Storage<T> for HeapStorage<T> {
    type SameSize<const M: usize> = HeapStorage<T>;

    #[inline]
    fn as_slice(&self) -> &[T] {
        &self.data
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    #[inline]
    fn zeros(len: usize) -> Self
    where
        T: Scalar,
    {
        Self {
            data: vec![T::zero(); len],
        }
    }
}
