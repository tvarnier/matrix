pub mod matrix;
pub mod vector;

use num_traits::Float;

pub fn lerp<V>(u: V, v: V, t: V) -> V
where
    V: Float
{
    return (v - u).mul_add(t, u);
}