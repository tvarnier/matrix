mod mat;
use mat::{matrix::Matrix, vector::Vector};

fn main() {
    let _ = mat::projection(120., 1., 2.0, 10.);
}
