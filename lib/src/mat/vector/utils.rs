use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float
        + std::default::Default
{
    pub fn sum_square(&self) -> K {
        let mut res: K = Default::default();
        for id in 0..self.array.len() {
            res = self.array[id].mul_add(self.array[id], res);
        }
        return res;
    }
}