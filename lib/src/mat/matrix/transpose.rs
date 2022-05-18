use super::Matrix;
use crate::Vector;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float + std::default::Default + std::fmt::Debug,
{
    pub fn transpose(&self) -> Matrix<K> {
        let mut res: Matrix<K> = Matrix::zero(self.col, self.row);
        for x in 0..self.row {
            for y in 0..self.col {
                res.array[y][x] = self.array[x][y];
            }
        }
        return res;
    }
}
