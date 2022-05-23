use super::Matrix;
use num_traits::Float;

impl<V> Matrix<V>
where
    V: Float + std::fmt::Debug + std::default::Default,
{
    pub fn lerp(u: Matrix<V>, v: Matrix<V>, t: V) -> Option<Matrix<V>> {
        if u.col == v.col && u.row == v.row {
            let mut res: Matrix<V> = Matrix::zero(u.row, u.col);
            for x in 0..u.array.len() {
                for y in 0..u.array[x].len() {
                    res.array[x][y] = (v.array[x][y] - u.array[x][y]).mul_add(t, u.array[x][y]);
                }
            }
            return Some(res);
        }
        return None;
    }
}

#[cfg(test)]
mod ex02 {
    use crate::matrix::Matrix;

    #[test]
    fn basic() {
        assert_eq!(
            Some(Matrix::from([[0.5, 5.], [-0.5, -5.]])),
            Matrix::lerp(
                Matrix::from([[0., 0.], [0., 0.]]),
                Matrix::from([[1., 10.], [-1., -10.]]),
                0.5
            )
        );
        assert_eq!(
            Some(Matrix::from([[11., 5.5], [16.5, 22.]])),
            Matrix::lerp(
                Matrix::from([[2., 1.], [3., 4.]]),
                Matrix::from([[20., 10.], [30., 40.]]),
                0.5
            )
        );
    }

    #[test]
    fn different_dimensions() {
        assert_eq!(
            None,
            Matrix::lerp(
                Matrix::from([[2., 1.], [3., 4.]]),
                Matrix::from([[20.], [30.]]),
                0.5
            )
        );
        assert_eq!(
            None,
            Matrix::lerp(
                Matrix::from([[2., 1.], [3., 4.]]),
                Matrix::from([[20., 10., 0.], [30., 40., 50.]]),
                0.5
            )
        );
        assert_eq!(
            None,
            Matrix::lerp(
                Matrix::from([[2., 1.], [3., 4.]]),
                Matrix::from([[20., 10.]]),
                0.5
            )
        );
        assert_eq!(
            None,
            Matrix::lerp(
                Matrix::from([[2., 1.], [3., 4.]]),
                Matrix::from([[20., 10.], [30., 40.], [50., 60.]]),
                0.5
            )
        );
    }
}
