use crate::Vector;
use core::ops::{Add, Index, IndexMut};
use core::simd::SimdElement;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Point<T: SimdElement, const D: usize>(Vector<T, D>);

impl<T: SimdElement, const D: usize> Point<T, D> {
    #[inline]
    pub const fn new(point: [T; D]) -> Self {
        Self(Vector::new(point))
    }

    #[inline]
    pub const fn into_inner(&self) -> [T; D] {
        self.0.into_inner()
    }
}

impl<T: SimdElement, const D: usize> From<[T; D]> for Point<T, D> {
    #[inline]
    fn from(point: [T; D]) -> Self {
        Self::new(point)
    }
}

impl<T: SimdElement, const D: usize> From<Vector<T, D>> for Point<T, D> {
    #[inline]
    fn from(vector: Vector<T, D>) -> Self {
        Self(vector)
    }
}

impl<T: SimdElement + Default, const D: usize> Default for Point<T, D> {
    #[inline]
    fn default() -> Self {
        Self(Vector::default())
    }
}

impl<T: SimdElement, const D: usize> Index<usize> for Point<T, D> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: SimdElement, const D: usize> IndexMut<usize> for Point<T, D> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const D: usize> Add for Point<f32, D> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

#[macro_export]
macro_rules! point {
    [$($x:expr),*] => {
        $crate::Point::new([$($x),*])
    };

    [$x:expr; $n:expr] => {
        $crate::Point::new([$x; $n])
    };
}
