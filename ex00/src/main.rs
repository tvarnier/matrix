mod structs;

use crate::structs::*;

fn main() {
    let mut v: Vector<f32> = Vector::new(vec![1., 2., 3.]);
    v.print();
    v.add(&Vector::<f32>::new(vec![2., 2., 3.]));
    v.print();
    v.sub(&Vector::<f32>::new(vec![2., 2., 3.]));
    v.print();
    v.scl(2.);
    v.print();
}