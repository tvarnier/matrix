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

#[cfg(test)]
mod ex13 {
    use crate::matrix::Matrix;

    #[test]
    fn basic() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(3, u.rank());
        let u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
        assert_eq!(2, u.rank());
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
        assert_eq!(3, u.rank());
    }
}
