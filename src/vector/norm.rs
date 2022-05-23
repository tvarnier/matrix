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

#[cfg(test)]
mod ex04 {
    use crate::vector::Vector;

    #[test]
    fn norm_1() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm_1(), 0.);

        let u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_1(), 6.);

        let u = Vector::from([-1., -2.]);
        assert_eq!(u.norm_1(), 3.);
    }

    #[test]
    fn norm() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm(), 0.);

        let u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm(), 3.7416573867739413);

        let u = Vector::from([-1., -2.]);
        assert_eq!(u.norm(), 2.23606797749979);
    }

    #[test]
    fn norm_inf() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm_inf(), 0.);

        let u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_inf(), 3.0);

        let u = Vector::from([-1., -2.]);
        assert_eq!(u.norm_inf(), 2.);
    }
}
