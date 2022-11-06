use std::ops;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, v: Self) -> Self {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = f32;

    fn mul(self, v: Self) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, v: f32) -> Self {
        Self {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, v: f32) -> Self {
        Self {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v,
        }
    }
}
