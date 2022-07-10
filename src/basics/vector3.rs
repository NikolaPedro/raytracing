use std::ops::{ Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign };


#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self { 
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self { 
            x: self.y * other.z - self.z * other.y, 
            y: self.z * other.x - self.x * other.z, 
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn squared_length(&self) -> f64 {
        self.dot(self)
    }
    
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn normalized(&self) -> Self {
        let length = self.length();
        Self { 
            x: self.x / length,
            y: self.y / length,
            z: self.z / length
         }
    }
}


impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Self) -> Self::Output {
        Self { 
            x: self.x + other.x, 
            y: self.y + other.y, 
            z: self.z + other.z
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vector3 {
    type Output= Vector3;

    fn sub(self, other: Self) -> Self::Output {
        Self { 
            x: self.x - other.x, 
            y: self.y - other.y, 
            z: self.z - other.z
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Self) -> Self::Output {
        self.cross(&other)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, value: f64) -> Self::Output {
        Self { 
            x: self.x * value,
            y: self.y * value,
            z: self.z * value
        }
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, value: f64) {
        self.x *= value;
        self.y *= value;
        self.z *= value;
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, value: f64) -> Self::Output {
        Self { 
            x: self.x / value,
            y: self.y /  value,
            z: self.z / value
        }
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, value: f64) {
        self.x /= value;
        self.y /= value;
        self.z /= value;
    }
}
