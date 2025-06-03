#![cfg_attr(not(feature = "std"), no_std)]
#![feature(repr_simd, portable_simd)]
#![feature(maybe_uninit_array_assume_init)]

mod matrix;
mod point;
mod vector;

pub mod ops;
pub mod simd;

pub use matrix::Matrix;
pub use point::Point;
pub use vector::Vector;
