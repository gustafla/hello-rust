use std::ops::{Add, Sub, Div, Mul};

#[derive(Debug, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Div for Vec2 {
    type Output = Vec2;
    fn div(self, other: Vec2) -> Vec2 {
        Vec2 {x: self.x / other.x, y: self.y / other.y}
    }
}

impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {x: self.x * other.x, y: self.y * other.y}
    }
}

impl Vec2 {
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn distance_from(&self, other: &Vec2) -> f32 {
        (self.clone() - other.clone()).length()
    }
}
