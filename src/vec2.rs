use std::ops::*;

#[derive(Default, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y }
    }
}


impl Vec2 {
    pub fn dot(&self, other: &Vec2) -> f64 {
        (self.x*other.x) + (self.y*other.y)
    }

    pub fn tripple_prod(v1: &Vec2, v2: &Vec2, v3: &Vec2) -> Vec2 {
        let cross_1_z = v1.x*v2.y - v1.y*v2.x;
        Vec2 { x: v3.y*cross_1_z, y: v3.x*cross_1_z }
    }

    pub fn length(&self) -> f64 {
        let squaresum = self.x.powi(2) + self.y.powi(2);
        squaresum.sqrt()
    }

    pub fn normalize(&self) -> Self {
        Vec2 {
            x: self.x/self.length(),
            y: self.y/self.length()
        }
    }

    pub fn origin() -> Self {
        Self::default()
    }
}
