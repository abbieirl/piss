pub trait LaneCount {
    const LANES: usize;
}

impl LaneCount for f32 {
    const LANES: usize = {
        if cfg!(target_feature = "avx512f") {
            16
        } else if cfg!(any(target_feature = "avx", target_feature = "avx2",)) {
            8
        } else if cfg!(any(
            target_feature = "sse4.1",
            target_feature = "ssse3",
            target_feature = "sse3",
            target_feature = "sse2",
            target_feature = "sse",
            target_feature = "neon",
            target_feature = "simd128",
        )) {
            4
        } else {
            1
        }
    };
}

impl LaneCount for f64 {
    const LANES: usize = {
        if cfg!(target_feature = "avx512f") {
            8
        } else if cfg!(any(target_feature = "avx", target_feature = "avx2",)) {
            4
        } else if cfg!(any(
            target_feature = "sse4.1",
            target_feature = "ssse3",
            target_feature = "sse3",
            target_feature = "sse2",
            target_feature = "sse",
            target_feature = "neon",
            target_feature = "simd128",
        )) {
            2
        } else {
            1
        }
    };
}
