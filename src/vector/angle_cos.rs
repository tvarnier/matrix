use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float + std::default::Default,
{
    pub fn angle_cos(u: &Vector<K>, v: &Vector<K>) -> K {
        if u.size == v.size {
            return u.dot(v) / (u.sum_square() * v.sum_square()).sqrt();
        }
        return Float::nan();
    }
}

#[cfg(test)]
mod ex05 {
    use crate::vector::Vector;

    #[test]
    fn basic() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);
        assert_eq!(1., Vector::angle_cos(&u, &v));
        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        assert_eq!(0., Vector::angle_cos(&u, &v));
        let u = Vector::from([-1., 1.]);
        let v = Vector::from([1., -1.]);
        assert_eq!(-1., Vector::angle_cos(&u, &v));
        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        assert_eq!(1., Vector::angle_cos(&u, &v));
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert_eq!(0.9746318461970762, Vector::angle_cos(&u, &v));
    }

    #[test]
    fn smaller_size() {
        let u: Vector<f64> = Vector::from([1., 0.]);
        let v: Vector<f64> = Vector::from([1.]);
        assert!(Vector::angle_cos(&u, &v).is_nan());
    }

    #[test]
    fn greater_size() {
        let u: Vector<f64> = Vector::from([1., 0.]);
        let v: Vector<f64> = Vector::from([1., 0., -1.]);
        assert!(Vector::angle_cos(&u, &v).is_nan());
    }
}
