use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float + std::ops::Mul<Output = K>,
{
    pub fn scl(&mut self, a: K) {
        for id in 0..self.array.len() {
            self.array[id] = self.array[id] * a;
        }
    }
}
