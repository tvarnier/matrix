use num_traits::ops::mul_add::MulAdd;

use std::convert::From;
use std::fmt;

#[derive(Clone)]
pub struct Vector<K>
where
    K: Copy + Clone
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
        + std::default::Default
        + MulAdd<Output = K>
        + std::fmt::Display,
{
    pub fn from<Arr: AsRef<[K]>>(array_values: Arr) -> Vector<K> {
        Vector {
            size: array_values.as_ref().len(),
            array: Vec::from(array_values.as_ref()),
        }
    }

    pub fn zero(size: usize) -> Vector<K> {
        Vector {
            size: size,
            array: vec![Default::default(); size]
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

    pub fn linear_combination(u: &[Vector<K>], coefs: &[K]) -> Option<Vector<K>> {
        if let Some(f) = u.first() {
            if u.len() == coefs.len() && u.iter().all(|x| x.size == f.size)
            {
                let mut res: Vector<K> = Vector::zero(f.size);
                for (i, vec) in u.iter().enumerate()
                {
                    for id in 0..res.array.len() {
                        res.array[id] = vec.array[id].mul_add(coefs[i], res.array[id]);
                    }
                }
                return Some(res);
            }
        }
        return None;
    }
}
