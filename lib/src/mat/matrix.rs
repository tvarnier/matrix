use num_traits::Float;
use std::fmt;

pub struct Matrix<K>
where
    K: Float
{
    pub array: Vec<Vec<K>>,
    pub row: usize,
    pub col: usize,
}

impl<K> fmt::Display for Matrix<K>
where
    K: Float 
        + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.array)
    }
}

impl<K> Matrix<K>
where
    K: Float
        + std::fmt::Debug
        + std::default::Default
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
        return Matrix::zero(0,0);
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
    }
}

mod add;
mod sub;
mod scl;
mod lerp;
mod mul;
mod trace;
mod transpose;