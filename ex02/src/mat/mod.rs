pub mod matrix;
pub mod vector;

use num_traits::ops::mul_add::MulAdd;

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: std::fmt::Debug
    + std::ops::Add<Output = V>
    + std::ops::Sub<Output = V>
    + std::ops::Mul<Output = V>
    + std::ops::Mul<Output = V>
    + Copy
    + ToString
    + std::fmt::Display
    + MulAdd<Output = V>
{
    return (v - u).mul_add(t, u);
}