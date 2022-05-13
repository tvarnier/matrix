use super::Matrix;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float
        + std::ops::Sub<Output = K>
{
    pub fn sub(&mut self, v: &Matrix<K>) {
        if self.size == v.size {
            for x in 0..self.array.len() {
                for y in 0..self.array[x].len() {
                    self.array[x][y] = self.array[x][y] - v.array[x][y];
                }
            }
        }
    }
}