use std::simd::f32x4;
fn main() {
    let a = f32x4(40.0, 41.0, 42.0, 43.0);
    let b = f32x4(1.0, 1.1, 3.4, 9.8);
    println!("{:?}", a + b);
}