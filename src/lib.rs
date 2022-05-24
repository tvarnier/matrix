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

    let scale: f32 = ((fov * std::f32::consts::PI / 180.) * 0.5).tan();
    proj_mat.array[0][0] = 1. / (ratio * scale); //scale the x coordinates of the projected point
    proj_mat.array[1][1] = 1. / scale; //scale the y coordinates of the projected point
    proj_mat.array[2][2] = -(far + near) / (far - near); //used to remap z to [0,1]
    proj_mat.array[3][2] = -(2. * far * near) / (far - near); //used to remap z [0,1]
    proj_mat.array[2][3] = -1.; //set w = -z

    return proj_mat;
}

#[cfg(test)]
mod ex02 {
    #[cfg(test)]
    mod lerp {
        use crate::lerp;

        #[test]
        fn basic() {
            assert_eq!(0., lerp(0., 1., 0.));
            assert_eq!(1., lerp(0., 1., 1.));
            assert_eq!(0.5, lerp(0., 1., 0.5));
            assert_eq!(27.3, lerp(21., 42., 0.3));
        }
    }
}

#[cfg(test)]
mod ex14 {
    #[cfg(test)]
    mod projection {
        use crate::projection;
        use crate::Matrix;

        #[test]
        fn basic() {
            assert_eq!(
                Matrix::from([
                    [0.57735026, 0.0, 0.0, 0.0],
                    [0.0, 0.57735026, 0.0, 0.0],
                    [0.0, 0.0, -1.2222222, -1.0],
                    [0.0, 0.0, -2.2222223, 0.0]
                ]),
                projection(120., 1., 1., 10.)
            );
        }
    }
}
