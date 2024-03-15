use crate::math::Vector3D;

#[derive(Debug, Clone, Copy)]
pub struct Transform3D {
    pub x_axis: Vector3D,
    pub y_axis: Vector3D,
    pub z_axis: Vector3D,
    pub position: Vector3D,
}
