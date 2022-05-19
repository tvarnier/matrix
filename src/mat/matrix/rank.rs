use super::Matrix;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float + std::fmt::Debug + std::default::Default,
{
    pub fn rank(&self) -> usize {
        let rref_mat: Matrix<K> = Matrix::row_echelon(self);
        let mut rank: usize = 0;
        for r in 0..rref_mat.row {
            if !(rref_mat.array[r].iter().all(|&x| x == Default::default())) {
                rank = rank + 1;
            }
        }
        return rank;
    }
}
