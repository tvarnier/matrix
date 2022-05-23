use super::Matrix;
use crate::vector::Vector;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float + std::fmt::Debug + std::default::Default,
{
    pub fn mul_vec(&self, vec: &Vector<K>) -> Option<Vector<K>> {
        if vec.size == self.col {
            let mut res: Vector<K> = Vector::zero(self.row);
            for x in 0..self.row {
                for y in 0..self.col {
                    res.array[x] = self.array[x][y].mul_add(vec.array[y], res.array[x]);
                }
            }
            return Some(res);
        }
        return None;
    }

    pub fn mul_mat(&self, mat: &Matrix<K>) -> Option<Matrix<K>> {
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
            return Some(res);
        }
        return None;
    }
}

#[cfg(test)]
mod ex07 {
    use crate::matrix::Matrix;
    use crate::vector::Vector;

    #[test]
    fn basic() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(Some(Vector::from([4., 2.])), u.mul_vec(&v));

        let u = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(Some(Vector::from([8., 4.])), u.mul_vec(&v));

        let u = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(Some(Vector::from([4., -4.])), u.mul_vec(&v));

        let u = Matrix::from([[2., -2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(Some(Vector::from([4.])), u.mul_vec(&v));

        let u = Matrix::from([[2., -2.], [-2., 2.], [1., -1.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(Some(Vector::from([4., -4., 2.])), u.mul_vec(&v));

        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(Some(Matrix::from([[1., 0.], [0., 1.]])), u.mul_mat(&v));

        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(Some(Matrix::from([[2., 1.], [4., 2.]])), u.mul_mat(&v));

        let u = Matrix::from([[3., -5.], [6., 8.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(Some(Matrix::from([[-14., -7.], [44., 22.]])), u.mul_mat(&v));

        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1.], [0.]]);
        assert_eq!(Some(Matrix::from([[1.0], [0.0]])), u.mul_mat(&v));

        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0., -1.], [0., 1., -1.]]);
        assert_eq!(
            Some(Matrix::from([[1., 0., -1.], [0., 1., -1.]])),
            u.mul_mat(&v)
        );
    }

    #[test]
    fn wrong_dimension() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4.]);
        assert_eq!(None, u.mul_vec(&v));

        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2., 0.]);
        assert_eq!(None, u.mul_vec(&v));

        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.]]);
        assert_eq!(None, u.mul_mat(&v));
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.], [0., 1.], [0., 0.]]);
        assert_eq!(None, u.mul_mat(&v));
    }
}
