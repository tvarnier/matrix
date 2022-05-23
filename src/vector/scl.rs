use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float + std::ops::Mul<Output = K>,
{
    pub fn scl(&mut self, a: K) {
        for id in 0..self.array.len() {
            self.array[id] = self.array[id] * a;
        }
    }
}

#[cfg(test)]
mod ex00 {
    use crate::vector::Vector;

    #[test]
    fn basic() {
        let mut u: Vector<f64> = Vector::from([2., 3.]);
        u.scl(2.);
        assert_eq!(u, Vector::from([4., 6.]));
    }
}
