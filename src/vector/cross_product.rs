use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float + std::default::Default + std::fmt::Debug,
{
    pub fn cross_product(u: &Vector<K>, v: &Vector<K>) -> Option<Vector<K>> {
        if v.size == 3 && u.size == 3 {
            let mut res: Vector<K> = Vector::zero(3);
            res.array[0] = u.array[1] * v.array[2] - u.array[2] * v.array[1];
            res.array[1] = u.array[2] * v.array[0] - u.array[0] * v.array[2];
            res.array[2] = u.array[0] * v.array[1] - u.array[1] * v.array[0];
            return Some(res);
        }
        return None;
    }
}

#[cfg(test)]
mod ex06 {
    use crate::vector::Vector;

    #[test]
    fn basic() {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0., 0.]);
        assert_eq!(
            Some(Vector::from([0., 1., 0.])),
            Vector::cross_product(&u, &v)
        );
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert_eq!(
            Some(Vector::from([-3., 6., -3.])),
            Vector::cross_product(&u, &v)
        );
        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([-2., -5., 16.]);
        assert_eq!(
            Some(Vector::from([17., -58., -16.])),
            Vector::cross_product(&u, &v)
        );
    }

    #[test]
    fn wrong_size() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([1., 0., 0.]);
        assert_eq!(None, Vector::cross_product(&u, &v));

        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0.]);
        assert_eq!(None, Vector::cross_product(&u, &v));
    }
}
