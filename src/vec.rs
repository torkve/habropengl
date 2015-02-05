use std::num::{Float, FromPrimitive, ToPrimitive, NumCast};
use std::ops::{Add, Sub, Mul, BitXor};
use std::clone::Clone;

#[derive(Clone, Debug)]
pub struct Vec2<T> where T: Clone + NumCast + Mul + Add + Sub {
    pub x: T,
    pub y: T
}

impl<T: Copy> Copy for Vec2<T> {}

impl<T: Copy + Clone + NumCast + Mul + Add + Sub> Vec2<T>
where <T as Mul>::Output: NumCast + Mul,
<<T as Mul>::Output as Mul>::Output: ToPrimitive {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 {x: x, y: y}
    }

    pub fn scale(&mut self, width: T, height: T) {
        self.x = NumCast::from(self.x * width * NumCast::from(0.5f32).unwrap()).unwrap();
        self.y = NumCast::from(self.y * height * NumCast::from(0.5f32).unwrap()).unwrap();
    }

    pub fn to<K: Clone + NumCast + Mul + Add + Sub>(&self) -> Vec2<K> {
        Vec2 {
            x: NumCast::from(self.x).unwrap(),
            y: NumCast::from(self.y).unwrap()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy> Copy for Vec3<T> {}

impl<T: Clone + NumCast + Mul + Add + Sub> Add for Vec2<T>
where <T as Add>::Output: Clone + NumCast + Mul + Add + Sub {
    type Output = Vec2<<T as Add<T>>::Output>;
    fn add(self, rhs: Vec2<T>) -> Vec2<<T as Add<T>>::Output> {
        Vec2{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl<T: Add<T>> Add for Vec3<T> {
    type Output = Vec3<<T as Add<T>>::Output>;
    fn add(self, rhs: Vec3<T>) -> Vec3<<T as Add<T>>::Output> {
        Vec3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl<T: Clone + NumCast + Mul + Add + Sub> Sub for Vec2<T>
where <T as Sub>::Output: Clone + NumCast + Mul + Add + Sub {
    type Output = Vec2<<T as Sub<T>>::Output>;
    fn sub(self, rhs: Vec2<T>) -> Vec2<<T as Sub<T>>::Output> {
        Vec2{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl<T: Sub> Sub for Vec3<T> {
    type Output = Vec3<<T as Sub<T>>::Output>;
    fn sub(self, rhs: Vec3<T>) -> Vec3<<T as Sub<T>>::Output> {
        Vec3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl<T: Clone + NumCast + Mul + Add + Sub + ToPrimitive + FromPrimitive> Mul<f32> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, rhs: f32) -> Vec2<T> {
        Vec2{
            x: FromPrimitive::from_f32(rhs * self.x.to_f32().unwrap()).unwrap(),
            y: FromPrimitive::from_f32(rhs * self.y.to_f32().unwrap()).unwrap(),
        }
    }
}

impl<T: Mul + FromPrimitive + ToPrimitive> Mul<f32> for Vec3<T> where <T as Mul>::Output: FromPrimitive {
    type Output = Vec3<<T as Mul<T>>::Output>;
    fn mul(self, rhs: f32) -> <Vec3<T> as Mul<f32>>::Output {
        Vec3{
            x: FromPrimitive::from_f32(rhs * self.x.to_f32().unwrap()).unwrap(),
            y: FromPrimitive::from_f32(rhs * self.y.to_f32().unwrap()).unwrap(),
            z: FromPrimitive::from_f32(rhs * self.z.to_f32().unwrap()).unwrap(),
        }
    }
}

impl<'a, T: Mul + FromPrimitive + ToPrimitive> Mul<f32> for &'a Vec3<T> where <T as Mul>::Output: FromPrimitive {
    type Output = Vec3<T>;
    fn mul(self, rhs: f32) -> Vec3<T> {
        Vec3{
            x: FromPrimitive::from_f32(rhs * self.x.to_f32().unwrap()).unwrap(),
            y: FromPrimitive::from_f32(rhs * self.y.to_f32().unwrap()).unwrap(),
            z: FromPrimitive::from_f32(rhs * self.z.to_f32().unwrap()).unwrap(),
        }
    }
}

impl<'a, T: Mul + Clone + Copy> BitXor for &'a Vec3<T> where <T as Mul>::Output: Sub {
    type Output = Vec3<<<T as Mul>::Output as Sub>::Output>;
    fn bitxor(self, rhs: &Vec3<T>) -> <Self as BitXor>::Output {
        let sx = self.x;
        let sy = self.y;
        let sz = self.z;
        let rx = rhs.x;
        let ry = rhs.y;
        let rz = rhs.z;
        Vec3{
            x: sy * rz - sz * ry,
            y: sz * rx - sx * rz,
            z: sx * ry - sy * rx
        }
    }
}

impl<T: Mul + Clone + Copy> BitXor for Vec3<T> where <T as Mul>::Output: Sub {
    type Output = Vec3<<<T as Mul>::Output as Sub>::Output>;
    fn bitxor(self, rhs: Vec3<T>) -> <Self as BitXor>::Output {
        let sx = self.x;
        let sy = self.y;
        let sz = self.z;
        let rx = rhs.x;
        let ry = rhs.y;
        let rz = rhs.z;
        Vec3{
            x: sy * rz - sz * ry,
            y: sz * rx - sx * rz,
            z: sx * ry - sy * rx
        }
    }
}

impl<T: Clone + Copy + NumCast + Mul + Add + Sub + FromPrimitive + ToPrimitive> Vec3<T>
where <T as Mul>::Output: Add + ToPrimitive + FromPrimitive + NumCast, <<T as Mul>::Output as Add>::Output: NumCast
{
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {x: x, y: y, z: z}
    }

    pub fn norm(&self) -> f32 {
        Float::sqrt((self.x * self.x).to_f32().unwrap() + (self.y * self.y).to_f32().unwrap() + (self.z * self.z).to_f32().unwrap())
    }

    pub fn vec_mul(&self, rhs: &Self) -> T {
        NumCast::from(self.x * rhs.x + NumCast::from(self.y * rhs.y + self.z * rhs.z).unwrap()).unwrap()
    }

    pub fn normalize(&self) -> Vec3<T> {
        let norm: f32 = 1f32 / self.norm();
        self * norm
    }
    
    pub fn to_vec2(&self) -> Vec2<T> {
        Vec2 {x: self.x, y: self.y}
    }
}

pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;
pub type Vec3f = Vec3<f32>;
pub type Vec3i = Vec3<i32>;
