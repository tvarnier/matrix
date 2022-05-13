use super::Vector;
use num_traits::Float;

impl<V> Vector<V>
where
    V: Float
        + std::fmt::Debug
        + std::default::Default
{
    pub fn lerp(u: Vector<V>, v: Vector<V>, t: V) -> Vector<V>
    {
        let mut res: Vector<V> = Vector::zero(u.size);
        if u.size == v.size {
            for id in 0..u.array.len() {
                res.array[id] = (v.array[id] - u.array[id]).mul_add(t, u.array[id]);
            }
        }
        return res;
    }
}