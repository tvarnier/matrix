pub mod matrix;
pub mod vector;

use num_traits::Float;

pub fn lerp<V>(u: V, v: V, t: V) -> V
where
    V: Float,
{
    return (v - u).mul_add(t, u);
}

use crate::matrix::Matrix;

pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    let mut proj_mat: Matrix<f32> = Matrix::zero(4, 4);

    let scale: f32 = (fov * 0.5 * std::f32::consts::PI / 180.).tan();
    proj_mat.array[0][0] = 1. / (ratio * scale); //scale the x coordinates of the projected point
    proj_mat.array[1][1] = 1. / scale; //scale the y coordinates of the projected point
    proj_mat.array[2][2] = -(far + near) / (far - near); //used to remap z to [0,1]
    proj_mat.array[3][2] = -(2. * far * near) / (far - near); //used to remap z [0,1]
    proj_mat.array[2][3] = -1.; //set w = -z

    return proj_mat;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("TESTTTTT");
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
