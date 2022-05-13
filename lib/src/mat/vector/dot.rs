use super::Vector;
use num_traits::Float;

impl<K> Vector::<K> 
where
    K: Float
        + std::default::Default
{
    pub fn dot(&self, v: Vector::<K>) -> K {
        let mut res: K = Default::default();
        if self.size == v.size {
            for id in 0..self.array.len() {
                res = self.array[id].mul_add(v.array[id], res);
            }
        }
        return res;
    }
}