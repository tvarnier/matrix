use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float
        + std::default::Default
{
    pub fn norm_1(&mut self) -> K {
        let mut res: K = Default::default();
        for id in 0..self.array.len() {
            res = res + self.array[id];
        }
        return res;
    }

    pub fn norm(&mut self) -> K {
        let mut res: K = Default::default();
        for id in 0..self.array.len() {
            match K::from(2.) {
                Some(p) => res = res + self.array[id].powf(p),
                None => return Float::nan(),
            }   
        }
        return res.sqrt();
    }

    pub fn norm_inf(&mut self) -> K {
        match self.array.iter().copied().fold(Float::neg_infinity(), Float::max_value()) {
            Some(max) => max,
            None => Float::nan(),
        }
    }
}


