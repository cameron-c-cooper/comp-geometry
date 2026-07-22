use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Index, IndexMut, Mul},
};

use crate::{HeapStorage, Scalar, StackStorage, Storage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix<T: Scalar, const R: usize, const C: usize, S> {
    pub storage: S,
    _marker: PhantomData<T>,
}

pub type SMatrix<T, const R: usize, const C: usize, const N: usize> =
    Matrix<T, R, C, StackStorage<T, N>>;

impl<T, const R: usize, const C: usize, S> Index<usize> for Matrix<T, R, C, S>
where
    S: Storage<T>,
    T: Scalar,
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
    T: Scalar,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < R, "row index out of bounds");
        let start = index * C;
        let end = start + C;
        &mut self.storage.as_mut_slice()[start..end]
    }
}

impl<T, const R: usize, const C: usize, S> AddAssign<&Matrix<T, R, C, S>> for Matrix<T, R, C, S>
where
    T: Scalar + AddAssign,
    S: Storage<T>,
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

impl<T, const R: usize, const C: usize, S> AddAssign<Matrix<T, R, C, S>> for Matrix<T, R, C, S>
where
    T: Scalar + AddAssign,
    S: Storage<T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: Matrix<T, R, C, S>) {
        *self += &rhs;
    }
}

impl<T, const R: usize, const C: usize, const N: usize> Add for SMatrix<T, R, C, N>
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

impl<T, const R: usize, const C: usize, const N: usize> Add<&SMatrix<T, R, C, N>>
    for SMatrix<T, R, C, N>
where
    T: Scalar,
    StackStorage<T, N>: Storage<T>,
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
            _marker: PhantomData,
        }
    }
}

impl<T, const R: usize, const C: usize, const N: usize> Add<&SMatrix<T, R, C, N>>
    for &SMatrix<T, R, C, N>
where
    T: Scalar,
    StackStorage<T, N>: Storage<T>,
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
            _marker: PhantomData,
        }
    }
}

#[macro_export]
macro_rules! smatrix {
    ( $( [ $( $x:expr ),* $(,)? ] ),* $(,)? ) => {{
        const R: usize = 0 $( + { let _ = [ $( stringify!($x) ),* ]; 1 } )*;
        const TOTAL: usize = 0 $( $( + { let _ = stringify!($x); 1 } )* )*;
        const C: usize = if R == 0 { 0 } else { TOTAL / R };

        let data = [ $( $( $x ),* ),* ];
        const N: usize = R * C;

        $crate::matrix::SMatrix::<_, R, C, N> {
            storage: $crate::matrix::StackStorage { data },
            _marker: std::marker::PhantomData,
        }
    }};
}

pub type HMatrix<T, const R: usize, const C: usize> = Matrix<T, R, C, HeapStorage<T>>;

impl<T, const R: usize, const C: usize> HMatrix<T, R, C>
where
    T: Scalar,
{
    pub fn from_vec(data: Vec<T>) -> Self {
        assert_eq!(
            data.len(),
            R * C,
            "Vector length {} does not match matrix size {}x{}",
            data.len(),
            R,
            C
        );
        Matrix {
            storage: HeapStorage { data },
            _marker: PhantomData,
        }
    }
    pub fn zeros() -> Self
    where
        T: Scalar,
    {
        Self::from_vec(vec![T::zero(); R * C])
    }
}

impl<T, const R: usize, const C: usize> Add<&HMatrix<T, R, C>> for HMatrix<T, R, C>
where
    T: Scalar,
    HeapStorage<T>: Storage<T>,
{
    type Output = Self;
    fn add(self, rhs: &HMatrix<T, R, C>) -> Self::Output {
        let len: usize = R * C;
        let mut data: Vec<T> = Vec::with_capacity(len);
        let lhs_slice = self.storage.as_slice();
        let rhs_slice = rhs.storage.as_slice();

        for i in 0..len {
            data.push(lhs_slice[i] + rhs_slice[i]);
        }
        Matrix {
            storage: HeapStorage { data },
            _marker: PhantomData,
        }
    }
}

impl<T, const R: usize, const C: usize> Add<&HMatrix<T, R, C>> for &HMatrix<T, R, C>
where
    T: Scalar,
    HeapStorage<T>: Storage<T>,
{
    type Output = HMatrix<T, R, C>;
    #[inline]
    fn add(self, rhs: &HMatrix<T, R, C>) -> Self::Output {
        let len: usize = R * C;
        let mut data: Vec<T> = Vec::with_capacity(len);
        let lhs_slice = self.storage.as_slice();
        let rhs_slice = rhs.storage.as_slice();

        for i in 0..len {
            data.push(lhs_slice[i] + rhs_slice[i]);
        }
        Matrix {
            storage: HeapStorage { data },
            _marker: PhantomData,
        }
    }
}

impl<T, const R: usize, const C: usize> Add for HMatrix<T, R, C>
where
    T: Scalar,
    HeapStorage<T>: Storage<T>,
{
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

#[macro_export]
macro_rules! hmatrix {
    ( $( [ $( $x:expr ),* $(,)? ] ),* $(,)? ) => {{
        const R: usize = 0 $( + { let _ = [ $( stringify!($x) ),* ]; 1 } )*;
        const TOTAL: usize = 0 $( $( + { let _ = stringify!($x); 1 } )* )*;
        const C: usize = if R == 0 { 0 } else { TOTAL / R };
        let data = vec![ $( $( $x ),* ),* ];

        $crate::matrix::HMatrix::<_, R, C> {
            storage: $crate::matrix::HeapStorage { data },
            _marker: std::marker::PhantomData,
        }
    }};
}

#[macro_export]
macro_rules! matrix {
    ( $($tt:tt)* ) => {
        $crate::smatrix!( $($tt)* )
    };
}

impl<T, const R: usize, const C: usize, S: Storage<T>> Matrix<T, R, C, S>
where
    T: Scalar,
    S: Storage<T>,
{
    pub fn transpose<SOut>(&self) -> Matrix<T, C, R, SOut>
    where
        SOut: Storage<T>,
    {
        let mut out_storage = SOut::zeros(R * C);
        let src = self.storage.as_slice();
        let dst = out_storage.as_mut_slice();
        for i in 0..R {
            for j in 0..C {
                dst[j * R + i] = src[i * C + j];
            }
        }
        Matrix {
            storage: out_storage,
            _marker: PhantomData,
        }
    }
}

impl<T, const R: usize, const C: usize, const N: usize, SLhs, SRhs> Mul<&Matrix<T, C, N, SRhs>>
    for &Matrix<T, R, C, SLhs>
where
    T: Scalar,
    SLhs: Storage<T>,
    SRhs: Storage<T>,
    SLhs::SameSize<{ R * N }>: Storage<T>,
{
    type Output = Matrix<T, R, N, SLhs::SameSize<{ R * N }>>;
    fn mul(self, rhs: &Matrix<T, C, N, SRhs>) -> Self::Output {
        let mut out_storage = SLhs::SameSize::<{ R * N }>::zeros(R * N);
        let lhs_slice = self.storage.as_slice();
        let rhs_slice = rhs.storage.as_slice();
        let out_slice = out_storage.as_mut_slice();

        for i in 0..R {
            let out_row = i * N;
            for k in 0..C {
                let lhs_val = lhs_slice[i * C + k];
                let rhs_row = k * N;
                for j in 0..N {
                    out_slice[out_row + j] += lhs_val * rhs_slice[rhs_row + j];
                }
            }
        }
        Matrix {
            storage: out_storage,
            _marker: PhantomData,
        }
    }
}

impl<T, const R: usize, const C: usize, const N: usize, SLhs, SRhs> Mul<&Matrix<T, C, N, SRhs>>
    for Matrix<T, R, C, SLhs>
where
    T: Scalar,
    SLhs: Storage<T>,
    SRhs: Storage<T>,
    SLhs::SameSize<{ R * N }>: Storage<T>,
    for<'a, 'b> &'a Matrix<T, R, C, SLhs>:
        Mul<&'b Matrix<T, C, N, SRhs>, Output = Matrix<T, R, N, SLhs::SameSize<{ R * N }>>>,
{
    type Output = Matrix<T, R, N, SLhs::SameSize<{ R * N }>>;

    #[inline]
    fn mul(self, rhs: &Matrix<T, C, N, SRhs>) -> Self::Output {
        &self * rhs
    }
}

// Matrix * Matrix
impl<T, const R: usize, const C: usize, const N: usize, SLhs, SRhs> Mul<Matrix<T, C, N, SRhs>>
    for Matrix<T, R, C, SLhs>
where
    T: Scalar,
    SLhs: Storage<T>,
    SRhs: Storage<T>,
    SLhs::SameSize<{ R * N }>: Storage<T>,
    for<'a, 'b> &'a Matrix<T, R, C, SLhs>:
        Mul<&'b Matrix<T, C, N, SRhs>, Output = Matrix<T, R, N, SLhs::SameSize<{ R * N }>>>,
{
    type Output = Matrix<T, R, N, SLhs::SameSize<{ R * N }>>;

    #[inline]
    fn mul(self, rhs: Matrix<T, C, N, SRhs>) -> Self::Output {
        &self * &rhs
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mat_add() {
        let smat_rhs = smatrix![[1, 2], [3, 4]];
        let smat_lhs = smatrix![[4, 3], [2, 1]];
        let golden_smat = smatrix![[5, 5], [5, 5]];
        assert_eq!(smat_rhs + smat_lhs, golden_smat);

        let hmat_rhs = hmatrix![[1, 2], [3, 4]];
        let hmat_lhs = hmatrix![[4, 3], [2, 1]];
        let golden_hmat = hmatrix![[5, 5], [5, 5]];
        assert_eq!(hmat_rhs + hmat_lhs, golden_hmat);
    }

    #[test]
    fn mat_transpose() {
        let mat = matrix![[1, 3], [4, 2]];
        let golden_mat = matrix![[1, 4], [3, 2]];
        assert_eq!(mat.transpose(), golden_mat);
    }

    #[test]
    fn mat_mul() {
        let mat_lhs = matrix![[4, 5, 7], [2, 1, 0]];
        let mat_rhs = matrix![[2, 3], [8, 9], [1, 1]];
        let golden_mat = matrix![[55, 64], [12, 15]];
        assert_eq!(mat_lhs * mat_rhs, golden_mat);
    }
}
