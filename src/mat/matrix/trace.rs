use super::Matrix;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float + std::default::Default + std::fmt::Debug,
{
    pub fn trace(&self) -> K {
        if self.is_square() {
            let mut res: K = Default::default();
            for x in 0..self.row {
                res = res + self.array[x][x];
            }
            return res;
        }
        return Float::nan();
    }
}
