use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float
        + std::ops::Add<Output = K>
        + std::fmt::Debug
        + std::default::Default
{
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