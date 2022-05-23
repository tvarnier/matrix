use super::Matrix;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float + std::ops::Add<Output = K>,
{
    pub fn add(&mut self, v: &Matrix<K>) {
        if self.col == v.col && self.row == v.row {
            for x in 0..self.array.len() {
                for y in 0..self.array[x].len() {
                    self.array[x][y] = self.array[x][y] + v.array[x][y];
                }
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
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u.add(&v);
        assert_eq!(Matrix::from([[8., 6.], [1., 6.]]), u);
    }

    #[test]
    fn smaller_row() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.]]);
        u.add(&v);
        assert_eq!(Matrix::from([[1., 2.], [3., 4.]]), u);
    }

    #[test]
    fn greater_row() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.], [2., 2.]]);
        u.add(&v);
        assert_eq!(Matrix::from([[1., 2.], [3., 4.]]), u);
    }

    #[test]
    fn smaller_col() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7.], [-2.]]);
        u.add(&v);
        assert_eq!(Matrix::from([[1., 2.], [3., 4.]]), u);
    }

    #[test]
    fn greater_col() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4., 42.], [-2., 2., 42.]]);
        u.add(&v);
        assert_eq!(Matrix::from([[1., 2.], [3., 4.]]), u);
    }
}
