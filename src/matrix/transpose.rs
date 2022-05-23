use super::Matrix;
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

#[cfg(test)]
mod ex09 {
    use crate::matrix::Matrix;

    #[test]
    fn basic() {
        let u: Matrix<f64> = Matrix::from([[0., 1.], [2., 3.], [4., 5.]]);
        assert_eq!(
            Matrix::from([[0.0, 2.0, 4.0], [1.0, 3.0, 5.0]]),
            u.transpose()
        );

        let u: Matrix<f64> = Matrix::from([[0.0, 2.0, 4.0], [1.0, 3.0, 5.0]]);
        assert_eq!(Matrix::from([[0., 1.], [2., 3.], [4., 5.]]), u.transpose());
    }
}
