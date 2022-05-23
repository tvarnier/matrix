use num_traits::Float;
use std::fmt;

#[derive(Debug)]
pub struct Vector<K>
where
    K: Float,
{
    pub array: Vec<K>,
    pub size: usize,
}

impl<K> PartialEq for Vector<K>
where
    K: Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.array == other.array
    }
}

impl<K> fmt::Display for Vector<K>
where
    K: Float + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("{:?}", self.array)
                .replace("[", "")
                .replace("]", "")
        )
    }
}

impl<K> Vector<K>
where
    K: Float + std::fmt::Debug + std::default::Default,
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
            array: vec![Default::default(); size],
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.array)
            .replace("[", "")
            .replace("]", "")
    }
}

mod add;
mod angle_cos;
mod cross_product;
mod dot;
mod lerp;
mod linear_combination;
mod norm;
mod scl;
mod sub;
mod utils;
