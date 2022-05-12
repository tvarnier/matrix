use std::convert::From;
use std::fmt;

#[derive(Clone)]
pub struct Vector<K>
where
    K: Copy,
{
    array: Vec<K>,
    pub size: usize,
}

impl<K> fmt::Display for Vector<K>
where
    K: Copy + ToString + std::fmt::Display + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.array)
    }
}

impl<K> Vector<K>
where
    K: std::fmt::Debug
        + std::ops::Add<Output = K>
        + std::ops::Sub<Output = K>
        + std::ops::Mul<Output = K>
        + Copy
        + ToString
        + std::fmt::Display,
{
    pub fn from<Arr: AsRef<[K]>>(array_values: Arr) -> Vector<K> {
        Vector {
            size: array_values.as_ref().len(),
            array: Vec::from(array_values.as_ref()),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.array)
    }

    pub fn add(&mut self, v: &Vector<K>) {
        if self.size == v.size {
            for id in 0..self.array.len() {
                self.array[id] = self.array[id] + v.array[id];
            }
        }
    }
    pub fn sub(&mut self, v: &Vector<K>) {
        if self.size == v.size {
            for id in 0..self.array.len() {
                self.array[id] = self.array[id] - v.array[id];
            }
        }
    }
    pub fn scl(&mut self, a: K) {
        for id in 0..self.array.len() {
            self.array[id] = self.array[id] * a;
        }
    }
}
