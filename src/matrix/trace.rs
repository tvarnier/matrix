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

#[cfg(test)]
mod ex08 {
    use crate::matrix::Matrix;

    #[test]
    fn basic() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(2., u.trace());
        let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(9., u.trace());
        let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert_eq!(-21., u.trace());
    }

    #[test]
    fn is_not_square() {
        let u: Matrix<f64> = Matrix::from([[1., 0., 0.], [0., 1., 0.]]);
        assert!(u.trace().is_nan());

        let u: Matrix<f64> = Matrix::from([[1.], [0.]]);
        assert!(u.trace().is_nan());
    }
}
