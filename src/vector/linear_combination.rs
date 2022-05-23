use super::Vector;
use num_traits::Float;

impl<K> Vector<K>
where
    K: Float + std::ops::Add<Output = K> + std::fmt::Debug + std::default::Default,
{
    pub fn linear_combination(u: &[Vector<K>], coefs: &[K]) -> Option<Vector<K>> {
        if let Some(f) = u.first() {
            if u.len() == coefs.len() && u.iter().all(|x| x.size == f.size) {
                let mut res: Vector<K> = Vector::zero(f.size);
                for (i, vec) in u.iter().enumerate() {
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

#[cfg(test)]
mod ex01 {
    use crate::vector::Vector;

    #[test]
    fn basic() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);

        assert_eq!(
            Some(Vector::from([10., -2., 0.5])),
            Vector::linear_combination(&[e1, e2, e3], &[10., -2., 0.5])
        );

        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);

        assert_eq!(
            Some(Vector::from([10., 0., 230.])),
            Vector::linear_combination(&[v1, v2], &[10., -2.])
        );
    }

    #[test]
    fn smaller_coef() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);

        assert_eq!(None, Vector::linear_combination(&[e1, e2, e3], &[10., -2.]));
    }

    #[test]
    fn greater_coef() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);

        assert_eq!(
            None,
            Vector::linear_combination(&[e1, e2, e3], &[10., -2., 0.5, 0.])
        );
    }
}
