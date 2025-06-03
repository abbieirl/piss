use piss::ops::Dot;
use piss::vector;

fn main() {
    dbg!(vector_from());
}

#[unsafe(no_mangle)]
fn vector_from() -> f32 {
    let a = divan::black_box(vector![1.0, 2.0, 3.0, 4.0]);
    let b = divan::black_box(vector![2.0; 4]);

    divan::black_box(a.dot(b))
}
