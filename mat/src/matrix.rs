use num_traits::Float;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Matrix<K>
where
    K: Float,
{
    pub array: Vec<Vec<K>>,
    pub row: usize,
    pub col: usize,
}

impl<K> PartialEq for Matrix<K>
where
    K: Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col && self.array == other.array
    }
}

impl<K> fmt::Display for Matrix<K>
where
    K: Float + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("{:?}", self.array)
                .replace("], [", "\n")
                .replace("[", "")
                .replace("]", "")
        )
    }
}

impl<K> Matrix<K>
where
    K: Float + std::fmt::Debug + std::default::Default,
{
    pub fn from<Arr: AsRef<[Row]>, Row: AsRef<[K]>>(array_values: Arr) -> Matrix<K> {
        if array_values.as_ref().len() > 0 {
            let s = array_values.as_ref()[0].as_ref().len();
            if array_values.as_ref().iter().all(|a| a.as_ref().len() == s) {
                return Matrix {
                    row: array_values.as_ref().len(),
                    col: s,
                    array: array_values
                        .as_ref()
                        .iter()
                        .map(|x| Vec::from(x.as_ref()))
                        .collect(),
                };
            }
        }
        return Matrix::zero(0, 0);
    }

    pub fn zero(row: usize, col: usize) -> Matrix<K> {
        Matrix {
            row: row,
            col: col,
            array: vec![vec![Default::default(); col]; row],
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.array)
            .replace("], [", "\n")
            .replace("[", "")
            .replace("]", "")
    }

    pub fn is_square(&self) -> bool {
        self.col == self.row
    }
}

mod add;
mod determinant;
mod inverse;
mod lerp;
mod mul;
mod rank;
mod row_echelon;
mod scl;
mod sub;
mod trace;
mod transpose;
