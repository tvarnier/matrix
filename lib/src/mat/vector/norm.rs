use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float + std::default::Default,
{
    pub fn norm_1(&self) -> K {
        let mut res: K = Default::default();
        for id in 0..self.array.len() {
            res = res + self.array[id].abs();
        }
        return res;
    }

    pub fn norm(&self) -> K {
        return self.sum_square().sqrt();
    }

    pub fn norm_inf(&self) -> K {
        self.array
            .iter()
            .map(|&x| x.abs())
            .fold(Float::neg_infinity(), |a, b| a.max(b))
    }
}
