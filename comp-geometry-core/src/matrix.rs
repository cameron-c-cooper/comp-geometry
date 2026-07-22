use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Index, IndexMut},
};

use crate::{Scalar, StackStorage, Storage};

pub struct Matrix<T: Scalar, const R: usize, const C: usize, S> {
    pub storage: S,
    _marker: PhantomData<T>,
}

pub type SMatrix<T, const R: usize, const C: usize, const N: usize> =
    Matrix<T, R, C, StackStorage<T, N>>;

impl<T, const R: usize, const C: usize, S> Index<usize> for Matrix<T, R, C, S>
where
    S: Storage<T>,
    T: Scalar
{
    type Output = [T];
    #[inline]
    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < R, "row index out of bounds");
        let start = row * C;
        let end = start + C;
        &self.storage.as_slice()[start..end]
    }
}

impl<T, const R: usize, const C: usize, S> IndexMut<usize> for Matrix<T, R, C, S>
where
    S: Storage<T>,
    T: Scalar
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < R, "row index out of bounds");
        let start = index * C;
        let end = start + C;
        &mut self.storage.as_mut_slice()[start..end]
    }
}

impl<T, const R: usize, const C: usize, S> AddAssign<&Matrix<T, R, C, S>>
for 
    Matrix<T, R, C, S>
where 
    T: Scalar + AddAssign,
    S: Storage<T>
{
    #[inline]
    fn add_assign(&mut self, rhs: &Matrix<T, R, C, S>) {
        let lhs_slice = self.storage.as_mut_slice();
        let rhs_slice = rhs.storage.as_slice();
        for (lhs_val, rhs_val) in lhs_slice.iter_mut().zip(rhs_slice.iter()) {
            *lhs_val += *rhs_val;
        }
    }
}

impl<T, const R: usize, const C: usize, S> AddAssign<Matrix<T, R, C, S>>
for 
    Matrix<T, R, C, S>
where 
    T: Scalar + AddAssign,
    S: Storage<T>
{
    #[inline]
    fn add_assign(&mut self, rhs: Matrix<T, R, C, S>) {
        *self += &rhs;
    }
}

impl<T, const R: usize, const C: usize, const N: usize> Add
for 
    SMatrix<T, R, C, N>
where 
    T: Scalar,
    StackStorage<T, N>: Storage<T>,
{
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<T, const R: usize, const C: usize, const N: usize> 
    Add<&SMatrix<T, R, C, N>>
for SMatrix<T, R, C, N>
where 
    T: Scalar,
    StackStorage<T, N>: Storage<T>
{
    type Output = Self;
    fn add(self, rhs: &SMatrix<T, R, C, N>) -> Self::Output {
        let mut data = [T::zero(); N];
        let lhs_slice = self.storage.as_slice();
        let rhs_slice = rhs.storage.as_slice();

        for i in 0..N {
            data[i] = lhs_slice[i] + rhs_slice[i];
        }
        Matrix {
            storage: StackStorage { data },
            _marker: PhantomData
        }
    }
}

impl<T, const R: usize, const C: usize, const N: usize> 
    Add<&SMatrix<T, R, C, N>>
for &SMatrix<T, R, C, N>
where 
    T: Scalar,
    StackStorage<T, N>: Storage<T>
{
    type Output = SMatrix<T, R, C, N>;
    #[inline]
    fn add(self, rhs: &SMatrix<T, R, C, N>) -> Self::Output {
        let mut data = [T::zero(); N];
        let lhs_slice = self.storage.as_slice();
        let rhs_slice = rhs.storage.as_slice();

        for i in 0..N {
            data[i] = lhs_slice[i] + rhs_slice[i];
        }
        Matrix {
            storage: StackStorage { data },
            _marker: PhantomData
        }
    }
}

#[macro_export]
macro_rules! matrix {
    ( $( [ $( $x:expr ),* $(,)? ] ),* $(,)? ) => {{
        const R: usize = 0 $( + { let _ = stringify!($x); 1 } )*;
        const C: usize = 0 $( + ${ignore($x)} 1 )*;
        let data = [ $( $( $x ),* ),* ];
        const N: usize = R * C;
        $crate::matrix::SMatrix::<_, R, C, N> {
            storage: $crate::matrix::StackStorage { data },
            _marker: std::marker::PhantomData,
        }
    }};
}

// TODO: Implement heap/dyn matrix, add some tests for mul, add, dot, etc
