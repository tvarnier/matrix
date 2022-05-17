use super::Matrix;
use num_traits::Float;

impl<V> Matrix<V>
where
    V: Float
        + std::fmt::Debug
        + std::default::Default
{
    pub fn lerp(u: Matrix<V>, v: Matrix<V>, t: V) -> Matrix<V>
    {
        let mut res: Matrix<V> = Matrix::zero(u.col, u.col);
        if u.col == v.col && u.col == v.col {
            for x in 0..u.array.len() {
                for y in 0..u.array[x].len() {
                    res.array[x][y] = (v.array[x][y] - u.array[x][y]).mul_add(t, u.array[x][y]);
                }
            }
        }
        return res;
    }
}