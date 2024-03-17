#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    x: f32,
    y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Vector2D) -> Vector2D {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &Vector2D) -> Vector2D {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}
