use crate::Point;
use crate::ops::{Dot, Sum};
use crate::simd::LaneCount;
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign};
use core::simd::num::SimdFloat;
use core::simd::{Simd, SimdElement};

#[repr(simd)]
#[derive(Debug, Clone, Copy)]
pub struct Vector<T: SimdElement, const D: usize>([T; D]);

impl<T: SimdElement, const D: usize> Vector<T, D> {
    #[inline]
    pub const fn new(vector: [T; D]) -> Self {
        Self(vector)
    }

    #[inline]
    pub const fn into_inner(&self) -> [T; D] {
        self.0
    }
}

impl<T: SimdElement, const D: usize> From<[T; D]> for Vector<T, D> {
    #[inline]
    fn from(vector: [T; D]) -> Self {
        Self::new(vector)
    }
}

impl<T: SimdElement, const D: usize> From<Point<T, D>> for Vector<T, D> {
    #[inline]
    fn from(point: Point<T, D>) -> Self {
        Self(point.into_inner())
    }
}

impl<T: SimdElement + Default, const D: usize> Default for Vector<T, D> {
    #[inline]
    fn default() -> Self {
        Self([T::default(); D])
    }
}

impl<T: SimdElement, const D: usize> Index<usize> for Vector<T, D> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: SimdElement, const D: usize> IndexMut<usize> for Vector<T, D> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const D: usize> Sum for Vector<f32, D> {
    type Output = f32;

    #[inline]
    fn sum(self) -> Self::Output {
        const LANES: usize = f32::LANES;

        let mut result = Simd::splat(0.0);
        let (chunks, remainder) = self.0.as_chunks::<LANES>();

        for &chunk in chunks {
            result += Simd::from_array(chunk);
        }

        if !remainder.is_empty() {
            let mut padded = [0.0; LANES];
            padded[..remainder.len()].copy_from_slice(remainder);
            result += Simd::from_array(padded);
        }

        result.reduce_sum()
    }
}

impl<const D: usize> Add for Vector<f32, D> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        const LANES: usize = f32::LANES;

        let mut result = [0.0; D];
        let result_chunks = result.as_chunks_mut::<LANES>().0;

        let (lhs_chunks, lhs_rem) = self.0.as_chunks::<LANES>();
        let (rhs_chunks, rhs_rem) = rhs.0.as_chunks::<LANES>();

        for ((&lhs, &rhs), out) in lhs_chunks
            .iter()
            .zip(rhs_chunks)
            .zip(result_chunks.iter_mut())
        {
            *out = (Simd::from_array(lhs) + Simd::from_array(rhs)).to_array();
        }

        for index in 0..lhs_rem.len() {
            result[D - lhs_rem.len() + index] = lhs_rem[index] + rhs_rem[index];
        }

        Self(result)
    }
}

impl<const D: usize> AddAssign for Vector<f32, D> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const D: usize> Mul for Vector<f32, D> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        const LANES: usize = f32::LANES;

        let mut result = [0.0; D];
        let result_chunks = result.as_chunks_mut::<LANES>().0;

        let (lhs_chunks, lhs_rem) = self.0.as_chunks::<LANES>();
        let (rhs_chunks, rhs_rem) = rhs.0.as_chunks::<LANES>();

        for ((&lhs, &rhs), out) in lhs_chunks
            .iter()
            .zip(rhs_chunks)
            .zip(result_chunks.iter_mut())
        {
            *out = (Simd::from_array(lhs) * Simd::from_array(rhs)).to_array();
        }

        for index in 0..lhs_rem.len() {
            result[D - lhs_rem.len() + index] = lhs_rem[index] * rhs_rem[index];
        }

        Self(result)
    }
}

impl<const D: usize> MulAssign for Vector<f32, D> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const D: usize> Dot for Vector<f32, D> {
    type Output = f32;

    #[inline]
    fn dot(self, rhs: Self) -> Self::Output {
        const LANES: usize = f32::LANES;

        let (lhs_chunks, lhs_rem) = self.0.as_chunks::<LANES>();
        let (rhs_chunks, rhs_rem) = rhs.0.as_chunks::<LANES>();

        let mut simd_sum = Simd::splat(0.0);

        for (&a, &b) in lhs_chunks.iter().zip(rhs_chunks.iter()) {
            let va = Simd::from_array(a);
            let vb = Simd::from_array(b);
            simd_sum += va * vb;
        }

        if !lhs_rem.is_empty() {
            let mut a_pad = [0.0; LANES];
            let mut b_pad = [0.0; LANES];

            a_pad[..lhs_rem.len()].copy_from_slice(lhs_rem);
            b_pad[..rhs_rem.len()].copy_from_slice(rhs_rem);

            let va = Simd::from_array(a_pad);
            let vb = Simd::from_array(b_pad);
            simd_sum += va * vb;
        }

        simd_sum.reduce_sum()
    }
} 

#[macro_export]
macro_rules! vector {
    [$($x:expr),*] => {
        $crate::Vector::new([$($x),*])
    };

    [$x:expr; $n:expr] => {
        $crate::Vector::new([$x; $n])
    };
}
