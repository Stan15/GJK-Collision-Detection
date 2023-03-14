use crate::vec2::Vec2;
use crate::shape::Shape;
pub trait Collidable {
    fn support_point(&self, direction: Vec2) -> Vec2;
}
pub trait CollisionMethod {
    fn has_collided(s1: Shape, s2: Shape) -> bool;
}
