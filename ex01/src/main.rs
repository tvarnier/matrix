mod mat;
use mat::{matrix::Matrix, vector::Vector};

fn main() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    if let Some(res) = Vector::linear_combination(&[e1, e2, e3], &[10., -2., 0.5]) {
        println!("{}", res);
    }
    // [10.]
    // [-2.]
    // [0.5]
    if let Some(res) = Vector::linear_combination(&[v1, v2], &[10., -2.]) {
        println!("{}", res);
    }
    // [10.]
    // [0.]
    // [230.]
}
