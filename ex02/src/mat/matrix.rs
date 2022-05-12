use std::convert::From;
use std::fmt;

pub struct Matrix<K>
where
    K: Copy,
{
    array: Vec<Vec<K>>,
    pub size: usize,
    pub length: usize,
}

impl<K> fmt::Display for Matrix<K>
where
    K: Copy + ToString + std::fmt::Display + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.array)
    }
}

impl<K> Matrix<K>
where
    K: std::fmt::Debug
        + std::ops::Add<Output = K>
        + std::ops::Sub<Output = K>
        + std::ops::Mul<Output = K>
        + Copy
        + ToString
        + std::fmt::Display,
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

    pub fn to_string(&self) -> String {
        format!("{:?}", self.array)
    }

    pub fn add(&mut self, v: &Matrix<K>) {
        if self.size == v.size && self.length == v.length {
            for x in 0..self.array.len() {
                for y in 0..self.array[x].len() {
                    self.array[x][y] = self.array[x][y] + v.array[x][y];
                }
            }
        }
    }
    pub fn sub(&mut self, v: &Matrix<K>) {
        if self.size == v.size {
            for x in 0..self.array.len() {
                for y in 0..self.array[x].len() {
                    self.array[x][y] = self.array[x][y] - v.array[x][y];
                }
            }
        }
    }
    pub fn scl(&mut self, a: K) {
        for x in 0..self.array.len() {
            for y in 0..self.array[x].len() {
                self.array[x][y] = self.array[x][y] * a;
            }
        }
    }
}
