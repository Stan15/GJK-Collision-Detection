use crate::{collision::*, vec2::Vec2};
pub struct Circle {
    pub center: Vec2,
    pub radius: f64
}

impl Shape for Circle {}
impl Collidable for Circle {
    fn support_point(&self, direction: Vec2) -> Vec2 {
        self.center + (direction.normalize()*self.radius)
    }
}
