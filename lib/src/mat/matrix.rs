use num_traits::Float;
use std::fmt;

pub struct Matrix<K>
where
    K: Float
{
    array: Vec<Vec<K>>,
    pub size: usize,
    pub length: usize,
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
        Matrix {
            size: array_values.as_ref().len(),
            length: array_values.as_ref().len(),
            array: array_values
                .as_ref()
                .iter()
                .map(|x| Vec::from(x.as_ref()))
                .collect(),
        }
    }

    pub fn zero(size: usize, length: usize) -> Matrix<K> {
        Matrix {
            size: size,
            length: length,
            array: vec![vec![Default::default(); size]; length],
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