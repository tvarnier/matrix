use num_traits::Float;
use std::fmt;

pub struct Vector<K>
where
    K: Float
{
    pub array: Vec<K>,
    pub size: usize,
}

impl<K> fmt::Display for Vector<K>
where
    K: Float
        + std::fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.array)
    }
}

impl<K> Vector<K>
where
    K: Float
        + std::fmt::Debug
        + std::default::Default
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
}

mod utils;
mod add;
mod sub;
mod scl;
mod linear_combination;
mod lerp;
mod dot;
mod norm;
mod angle_cos;
mod cross_product;