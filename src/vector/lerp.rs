use super::Vector;
use num_traits::Float;

impl<V> Vector<V>
where
    V: Float + std::fmt::Debug + std::default::Default,
{
    pub fn lerp(u: Vector<V>, v: Vector<V>, t: V) -> Option<Vector<V>> {
        if u.size == v.size {
            let mut res: Vector<V> = Vector::zero(u.size);
            for id in 0..u.array.len() {
                res.array[id] = (v.array[id] - u.array[id]).mul_add(t, u.array[id]);
            }
            return Some(res);
        }
        return None;
    }
}

#[cfg(test)]
mod ex02 {
    use crate::vector::Vector;

    #[test]
    fn basic() {
        assert_eq!(
            Some(Vector::from([0.5, 5.])),
            Vector::lerp(Vector::from([0., 0.]), Vector::from([1., 10.]), 0.5)
        );
        assert_eq!(
            Some(Vector::from([2.6, 1.3])),
            Vector::lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3)
        );
    }

    #[test]
    fn different_size() {
        assert_eq!(
            None,
            Vector::lerp(Vector::from([2., 1.]), Vector::from([4.]), 0.3)
        );
        assert_eq!(
            None,
            Vector::lerp(Vector::from([2., 1.]), Vector::from([4., 2., 0.]), 0.3)
        );
    }
}
