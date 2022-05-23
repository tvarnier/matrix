use super::Matrix;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float + std::fmt::Debug + std::default::Default,
{
    pub fn row_echelon(&self) -> Matrix<K> {
        let mut res: Matrix<K> = self.clone();

        let mut lead: usize = 0;
        let row_count: usize = self.row;
        let col_count: usize = self.col;

        for r in 0..row_count {
            if col_count <= lead {
                return res;
            }
            let mut i = r;
            while res.array[i][lead] == Default::default() {
                i = i + 1;
                if row_count == i {
                    i = r;
                    lead = lead + 1;
                    if col_count == lead {
                        return res;
                    }
                }
            }

            if i != r {
                res.array.swap(i, r);
            }

            let div: K = res.array[r][lead];
            for x in 0..res.array[r].len() {
                res.array[r][x] = res.array[r][x] / div;
            }

            for i in 0..row_count {
                if i != r {
                    let l: K = res.array[i][lead];
                    for x in 0..res.array[i].len() {
                        res.array[i][x] = res.array[i][x] - l * res.array[r][x];
                    }
                }
            }

            lead = lead + 1;
        }
        return res;
    }
}

#[cfg(test)]
mod ex10 {
    use crate::matrix::Matrix;

    #[test]
    fn basic() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(
            Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]),
            u.row_echelon(),
        );
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        assert_eq!(Matrix::from([[1.0, 0.0], [0.0, 1.0]]), u.row_echelon());
        let u = Matrix::from([[1., 2.], [2., 4.]]);
        assert_eq!(Matrix::from([[1.0, 2.0], [0.0, 0.0]]), u.row_echelon());
        let u = Matrix::from([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        assert_eq!(
            Matrix::from([
                [1.0, 0.625, 0.0, 0.0, -12.166666666666668],
                [0.0, 0.0, 1.0, 0.0, -3.666666666666667],
                [0.0, 0.0, 0.0, 1.0, 29.500000000000004],
            ]),
            u.row_echelon(),
        );
    }
}
