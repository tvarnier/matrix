use super::Matrix;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float + std::fmt::Debug + std::default::Default + std::fmt::Display,
{
    pub fn determinant(&self) -> K {
        if self.row == self.col {
            if self.row == 1 {
                return self.array[0][0];
            }
            let mut sub_mat: Matrix<K> = Matrix::zero(self.row - 1, self.col - 1);
            let mut res: K = Default::default();
            for id in 0..self.col {
                for x in 1..self.row {
                    for y in 0..sub_mat.col {
                        sub_mat.array[x - 1][y] = if y >= id {
                            self.array[x][y + 1]
                        } else {
                            self.array[x][y]
                        };
                    }
                }
                res = if id % 2 == 0 {
                    self.array[0][id].mul_add(sub_mat.determinant(), res)
                } else {
                    self.array[0][id].mul_add(-(sub_mat.determinant()), res)
                };
            }
            return res;
        }
        return Float::nan();
    }
}
