use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float
    + std::default::Default
{
    pub fn angle_cos(u: &Vector<K>, v: &Vector<K>) -> K {
        u.dot(v) / (u.sum_square() * v.sum_square()).sqrt()
        //u.dot(v) / u.norm().mul_add(v.norm(), Default::default())
    }
}

