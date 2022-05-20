use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float + std::default::Default + std::fmt::Debug,
{
    pub fn cross_product(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
        let mut res: Vector<K> = Vector::zero(3);
        if u.size == v.size && u.size == 3 {
            res.array[0] = u.array[1] * v.array[2] - u.array[2] * v.array[1];
            res.array[1] = u.array[2] * v.array[0] - u.array[0] * v.array[2];
            res.array[2] = u.array[0] * v.array[1] - u.array[1] * v.array[0];
        }
        return res;
    }
}
