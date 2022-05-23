use super::Matrix;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float + std::fmt::Debug + std::default::Default,
{
    fn from_float(f: f64) -> K {
        match K::from(f) {
            Some(res) => res,
            _ => Default::default(),
        }
    }

    pub fn inverse(&self) -> Option<Matrix<K>> {
        if self.is_square() {
            let mut augmented_mat: Matrix<K> = Matrix::zero(self.row, self.col * 2);
            for r in 0..self.row {
                for c in 0..(self.col * 2) {
                    if c < self.col {
                        augmented_mat.array[r][c] = self.array[r][c];
                    } else {
                        augmented_mat.array[r][c] = if c == self.col + r {
                            Matrix::from_float(1.)
                        } else {
                            Matrix::from_float(0.)
                        };
                    }
                }
            }

            augmented_mat = augmented_mat.row_echelon();

            let mut res: Matrix<K> = Matrix::zero(self.row, self.col);
            for r in 0..self.row {
                for c in 0..self.col {
                    res.array[r][c] = augmented_mat.array[r][c + self.col];
                }
            }
            return Some(res);
        }
        return None;
    }
}

#[cfg(test)]
mod ex12 {
    use crate::matrix::Matrix;

    #[test]
    fn basic() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(
            Some(Matrix::from([
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ])),
            u.inverse()
        );
        let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        assert_eq!(
            Some(Matrix::from([
                [0.5, 0.0, 0.0],
                [0.0, 0.5, 0.0],
                [0.0, 0.0, 0.5]
            ])),
            u.inverse()
        );

        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        assert_eq!(
            Some(Matrix::from([
                [0.6494252873563219, 0.09770114942528735, -0.6551724137931034],
                [
                    -0.7816091954022988,
                    -0.12643678160919541,
                    0.9655172413793103
                ],
                [
                    0.14367816091954022,
                    0.07471264367816091,
                    -0.2068965517241379
                ]
            ])),
            u.inverse()
        );
    }

    #[test]
    fn is_not_square() {
        let u: Matrix<f64> = Matrix::from([[1.], [-1.]]);
        assert_eq!(None, u.inverse());

        let u: Matrix<f64> = Matrix::from([[1., 2., 3.], [-1., -2., -3.]]);
        assert_eq!(None, u.inverse());
    }
}
