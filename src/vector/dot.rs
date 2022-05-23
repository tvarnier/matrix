use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float + std::default::Default,
{
    pub fn dot(&self, v: &Vector<K>) -> K {
        if self.size == v.size {
            let mut res: K = Default::default();
            for id in 0..self.array.len() {
                res = self.array[id].mul_add(v.array[id], res);
            }
            return res;
        }
        return Float::nan();
    }
}

#[cfg(test)]
mod ex03 {
    use crate::vector::Vector;

    #[test]
    fn basic() {
        let u: Vector<f64> = Vector::from([0., 0.]);
        let v: Vector<f64> = Vector::from([1., 1.]);
        assert_eq!(u.dot(&v), 0.);

        let u: Vector<f64> = Vector::from([1., 1.]);
        let v: Vector<f64> = Vector::from([1., 1.]);
        assert_eq!(u.dot(&v), 2.);

        let u: Vector<f64> = Vector::from([-1., 6.]);
        let v: Vector<f64> = Vector::from([3., 2.]);
        assert_eq!(u.dot(&v), 9.);
    }

    #[test]
    fn smaller_size() {
        let u: Vector<f64> = Vector::from([1., 1.]);
        let v: Vector<f64> = Vector::from([1.]);
        assert!(u.dot(&v).is_nan());
    }

    #[test]
    fn greater_size() {
        let u: Vector<f64> = Vector::from([1., 1.]);
        let v: Vector<f64> = Vector::from([1., 1., 1.]);
        assert!(u.dot(&v).is_nan());
    }
}
