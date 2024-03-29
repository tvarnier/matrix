use super::Matrix;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float + std::ops::Mul<Output = K>,
{
    pub fn scl(&mut self, a: K) {
        for x in 0..self.array.len() {
            for y in 0..self.array[x].len() {
                self.array[x][y] = self.array[x][y] * a;
            }
        }
    }
}

#[cfg(test)]
mod ex00 {
    use crate::matrix::Matrix;

    #[test]
    fn basic() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        u.scl(2.);
        assert_eq!(Matrix::from([[2., 4.], [6., 8.]]), u);
    }
}
