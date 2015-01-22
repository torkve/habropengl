use std::num::{Float, FromPrimitive};
use std::ops::{Add, Sub, Mul, BitXor};

#[derive(Clone, Show)]
pub struct Vec2i {
    pub x: isize,
    pub y: isize
}

impl Vec2i {
    pub fn new(x: isize, y: isize) -> Vec2i {
        Vec2i {x: x, y: y}
    }
}

#[derive(Clone, Show)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f {x: x, y: y}
    }
}

pub trait Vec2 {}
impl Vec2 for Vec2i {}
impl Vec2 for Vec2f {}

#[derive(Clone, Show)]
pub struct Vec3i {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Vec3i {
    pub fn new(x: isize, y: isize, z: isize) -> Vec3i {
        Vec3i {x: x, y: y, z: z}
    }
}

#[derive(Clone, Show)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f {x: x, y: y, z: z}
    }
}

pub trait Vec3 {
    fn norm(&self) -> f32;
    fn normalize(&mut self);
}

impl Vec3 for Vec3i {
    fn norm(&self) -> f32 {Float::sqrt((self.x * self.x + self.y * self.y + self.z * self.z) as f32)}
    fn normalize(&mut self) {
        let norm = 1f32 / self.norm();
        *self = self.clone() * norm;
    }
}

impl Vec3 for Vec3f {
    fn norm(&self) -> f32 {(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()}
    fn normalize(&mut self) {
        let norm = 1f32 / self.norm();
        *self = self.clone() * norm;
    }
}

impl Add for Vec2i {
    type Output = Vec2i;
    fn add(self, rhs: Vec2i) -> Vec2i {
        Vec2i{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Add for Vec2f {
    type Output = Vec2f;
    fn add(self, rhs: Vec2f) -> Vec2f {
        Vec2f{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Add for Vec3i {
    type Output = Vec3i;
    fn add(self, rhs: Vec3i) -> Vec3i {
        Vec3i{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Add for Vec3f {
    type Output = Vec3f;
    fn add(self, rhs: Vec3f) -> Vec3f {
        Vec3f{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z}
    }
}

impl Sub for Vec2i {
    type Output = Vec2i;
    fn sub(self, rhs: Vec2i) -> Vec2i {
        Vec2i{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Sub for Vec2f {
    type Output = Vec2f;
    fn sub(self, rhs: Vec2f) -> Vec2f {
        Vec2f{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Sub for Vec3i {
    type Output = Vec3i;
    fn sub(self, rhs: Vec3i) -> Vec3i {
        Vec3i{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;
    fn sub(self, rhs: Vec3f) -> Vec3f {
        Vec3f{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl Mul<f32> for Vec2i {
    type Output = Vec2i;
    fn mul(self, rhs: f32) -> Vec2i {
        Vec2i{x: (rhs * FromPrimitive::from_int(self.x).unwrap()) as isize, y: (rhs * FromPrimitive::from_int(self.y).unwrap()) as isize}
    }
}

impl Mul<f32> for Vec2f {
    type Output = Vec2f;
    fn mul(self, rhs: f32) -> Vec2f {
        Vec2f{x: rhs * self.x, y: rhs * self.y}
    }
}

impl Mul<f32> for Vec3i {
    type Output = Vec3i;
    fn mul(self, rhs: f32) -> Vec3i {
        Vec3i{x: (rhs * FromPrimitive::from_int(self.x).unwrap()) as isize, y: (rhs * FromPrimitive::from_int(self.y).unwrap()) as isize, z: (rhs * FromPrimitive::from_int(self.z).unwrap()) as isize}
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: f32) -> Vec3f {
        Vec3f{x: rhs * self.x, y: rhs * self.y, z: rhs * self.z}
    }
}

impl Mul for Vec3i {
    type Output = isize;
    fn mul(self, rhs: Vec3i) -> isize {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul for Vec3f {
    type Output = f32;
    fn mul(self, rhs: Vec3f) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl BitXor for Vec3i {
    type Output = Vec3i;
    fn bitxor(self, rhs: Vec3i) -> Vec3i {
        Vec3i{x: self.y * rhs.z - self.z * rhs.y, y: self.z * rhs.x - self.x * rhs.z, z: self.x * rhs.y - self.y * rhs.x}
    }
}

impl BitXor for Vec3f {
    type Output = Vec3f;
    fn bitxor(self, rhs: Vec3f) -> Vec3f {
        Vec3f{x: self.y * rhs.z - self.z * rhs.y, y: self.z * rhs.x - self.x * rhs.z, z: self.x * rhs.y - self.y * rhs.x}
    }
}
