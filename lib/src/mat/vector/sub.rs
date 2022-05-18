use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float + std::ops::Sub<Output = K>,
{
    pub fn sub(&mut self, v: &Vector<K>) {
        if self.size == v.size {
            for id in 0..self.array.len() {
                self.array[id] = self.array[id] - v.array[id];
            }
        }
    }
}
