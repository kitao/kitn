use crate::math::Vector2D;

#[derive(Debug, Clone, Copy)]
pub struct Transform2D {
    pub x_axis: Vector2D,
    pub y_axis: Vector2D,
    pub position: Vector2D,
}
