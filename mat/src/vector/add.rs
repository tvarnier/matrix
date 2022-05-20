use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float + std::ops::Add<Output = K>,
{
    pub fn add(&mut self, v: &Vector<K>) {
        if self.size == v.size {
            for id in 0..self.array.len() {
                self.array[id] = self.array[id] + v.array[id];
            }
        }
    }
}

#[cfg(test)]
mod ex00 {
    use crate::vector::Vector;

    #[test]
    fn add() {
        let mut u: Vector<f64> = Vector::from([2., 3.]);
        let v: Vector<f64> = Vector::from([5., 7.]);
        u.add(&v);
        assert_eq!(u, Vector::from([7., 10.]));
    }
}
