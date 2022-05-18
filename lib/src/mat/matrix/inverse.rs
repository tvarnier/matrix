use super::Matrix;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float + std::fmt::Debug + std::default::Default,
{
    fn fromFloat(f: f64) -> K {
        match K::from(f) {
            Some(res) => res,
            _ => Default::default(),
        }
    }

    pub fn inverse(&self) -> Result<Matrix<K>, String> {
        if self.row == self.col {
            let mut augmented_mat: Matrix<K> = Matrix::zero(self.row, self.col * 2);
            for r in 0..self.row {
                for c in 0..(self.col * 2) {
                    if c < self.col {
                        augmented_mat.array[r][c] = self.array[r][c];
                    } else {
                        augmented_mat.array[r][c] = if c == self.col + r {
                            Matrix::fromFloat(1.)
                        } else {
                            Matrix::fromFloat(0.)
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
            return Ok(res);
        }
        return Err("Error".to_string());
    }
}
