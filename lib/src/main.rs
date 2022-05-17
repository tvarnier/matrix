mod mat;
use mat::{matrix::Matrix, vector::Vector};

fn main() {
    let u = Matrix::from([
        [1., 0.],
        [0., 1.],
    ]);
    println!("{}", u.transpose());
    // 2.0
    let u = Matrix::from([
        [2., -5., 0.],
        [4., 3., 7.],
        [-2., 3., 4.],
    ]);
    println!("{}", u.transpose());
    // 9.0
    let u = Matrix::from([
        [1., 2.],
        [3., 4.],
        [5., 6.],
    ]);
    println!("{}", u.transpose());
    // -21.0
}