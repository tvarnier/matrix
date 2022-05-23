use super::Matrix;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float + std::fmt::Debug + std::default::Default + std::fmt::Display,
{
    pub fn determinant(&self) -> K {
        if self.is_square() {
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

#[cfg(test)]
mod ex11 {
    use crate::matrix::Matrix;

    #[test]
    fn basic() {
        let u = Matrix::from([[1., -1.], [-1., 1.]]);
        assert_eq!(0., u.determinant());
        // 0.0
        let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        assert_eq!(8., u.determinant());
        // 8.0
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        assert_eq!(-174., u.determinant());
        // -174.0
        let u = Matrix::from([
            [8., 5., -2., 4.],
            [4., 2.5, 20., 4.],
            [8., 5., 1., 4.],
            [28., -4., 17., 1.],
        ]);
        assert_eq!(1032., u.determinant());
        // 1032
    }

    #[test]
    fn is_not_square() {
        let u: Matrix<f64> = Matrix::from([[1.], [-1.]]);
        assert!(u.determinant().is_nan());

        let u: Matrix<f64> = Matrix::from([[1., 2., 3.], [-1., -2., -3.]]);
        assert!(u.determinant().is_nan());
    }
}
