use super::Matrix;
use num_traits::Float;

impl<K> Matrix<K>
where
    K: Float
        + std::fmt::Debug
        + std::default::Default
{
    pub fn row_echelon(&self) -> Matrix<K> {
        let mut res: Matrix<K> = self.clone();

        let mut lead: usize = 0;
        let mut row_count: usize = self.row;
        let mut col_count: usize = self.col;

        for r in 0..row_count {
            if col_count <= lead { return res; }
            let mut i = r;
            while res.array[i][lead] == Default::default() {
                i = i + 1;
                if row_count == i  {
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
                    let l : K = res.array[i][lead];
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