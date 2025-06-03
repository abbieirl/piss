use crate::Vector;
use core::mem::MaybeUninit;
use core::ops::{Index, IndexMut};
use core::simd::SimdElement;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix<T: SimdElement, const R: usize, const C: usize>([Vector<T, R>; C]);

impl<T: SimdElement, const R: usize, const C: usize> Matrix<T, R, C> {
    #[inline]
    pub const fn new(matrix: [[T; R]; C]) -> Self {
        let mut data = [MaybeUninit::uninit(); C];

        let mut index = 0;
        while index < C {
            data[index].write(Vector::new(matrix[index]));
            index += 1;
        }

        // SAFETY: The array is guaranteed to be in an initalized state.
        Self(unsafe { MaybeUninit::array_assume_init(data) })
    }
}

impl<T: SimdElement, const R: usize, const C: usize> From<[[T; R]; C]> for Matrix<T, R, C> {
    #[inline]
    fn from(matrix: [[T; R]; C]) -> Self {
        Self::new(matrix)
    }
}

impl<T: SimdElement + Default, const R: usize, const C: usize> Default for Matrix<T, R, C> {
    #[inline]
    fn default() -> Self {
        Self([Vector::default(); C])
    }
}

impl<T: SimdElement, const R: usize, const C: usize> Index<usize> for Matrix<T, R, C> {
    type Output = Vector<T, R>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: SimdElement, const R: usize, const C: usize> IndexMut<usize> for Matrix<T, R, C> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[macro_export]
macro_rules! matrix {
    [$([$($x:expr),*]),*] => {
        $crate::Matrix::new([$([$($x),*]),*])
    };
}