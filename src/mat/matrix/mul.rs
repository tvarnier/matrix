use super::Matrix;
use crate::Vector;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float + std::fmt::Debug + std::default::Default,
{
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        if vec.size == self.col {
            let mut res: Vector<K> = Vector::zero(self.row);
            for x in 0..self.row {
                for y in 0..self.col {
                    res.array[x] = self.array[x][y].mul_add(vec.array[y], res.array[x]);
                }
            }
            return res;
        }
        return Vector::zero(0);
    }

    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        if self.col == mat.row {
            let mut res: Matrix<K> = Matrix::zero(self.row, mat.col);
            for x in 0..self.row {
                for y in 0..mat.col {
                    for z in 0..self.col {
                        res.array[x][y] =
                            self.array[x][z].mul_add(mat.array[z][y], res.array[x][y]);
                    }
                }
            }
            return res;
        }
        return Matrix::zero(0, 0);
    }
}
