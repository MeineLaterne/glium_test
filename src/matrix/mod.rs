use std::ops::{Add, Mul, MulAssign};

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct Matrix4<T> where T:Copy {
    m11:T, m12:T, m13:T, m14:T,
    m21:T, m22:T, m23:T, m24:T,
    m31:T, m32:T, m33:T, m34:T,
    m41:T, m42:T, m43:T, m44:T,
}

impl Matrix4<f32> {

    pub fn new() -> Matrix4<f32> {
        Matrix4 {
            m11: 1.0, m12: 0.0, m13: 0.0, m14: 0.0,
            m21: 0.0, m22: 1.0, m23: 0.0, m24: 0.0,
            m31: 0.0, m32: 0.0, m33: 1.0, m34: 0.0,
            m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
        }
    }

    pub fn translation(x:f32, y:f32, z:f32) -> Matrix4<f32> {
        Matrix4 {
            m11: 1.0, m12: 0.0, m13: 0.0, m14: 0.0,
            m21: 0.0, m22: 1.0, m23: 0.0, m24: 0.0,
            m31: 0.0, m32: 0.0, m33: 1.0, m34: 0.0,
            m41: x, m42: y, m43: z, m44: 1.0,
        }
    }

    pub fn scale(x:f32, y:f32, z:f32) -> Matrix4<f32> {
        Matrix4 {
            m11: x, m12: 0.0, m13: 0.0, m14: 0.0,
            m21: 0.0, m22: y, m23: 0.0, m24: 0.0,
            m31: 0.0, m32: 0.0, m33: z, m34: 0.0,
            m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
        }
    }

    pub fn rotation_2d(angle:f32) -> Matrix4<f32> {
        Matrix4 {
            m11: angle.cos(), m12: angle.sin(), m13: 0.0, m14: 0.0,
            m21: -angle.sin(), m22: angle.cos(), m23: 0.0, m24: 0.0,
            m31: 0.0, m32: 0.0, m33: 1.0, m34: 0.0,
            m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
        }
    }

    pub fn orthographic(left:f32, right:f32, bottom:f32, top:f32, near_plane:f32, far_plane:f32) -> Matrix4<f32> {
        Matrix4 {
            m11: 2.0 / (right - left), m12: 0.0, m13: 0.0, m14: 0.0,
            m21: 0.0, m22: 2.0 / (top - bottom), m23: 0.0, m24: 0.0,
            m31: 0.0, m32: 0.0, m33: 1.0 / (near_plane - far_plane), m34: 0.0,
            m41: (left + right) / (left - right), m42: (top + bottom) / (bottom - top), 
            m43: near_plane / (near_plane - far_plane), m44: 1.0,
        }
    }

    pub fn to_array(&self) -> [[f32; 4]; 4] {
        [
            [self.m11, self.m12, self.m13, self.m14],
            [self.m21, self.m22, self.m23, self.m24],
            [self.m31, self.m32, self.m33, self.m34],
            [self.m41, self.m42, self.m43, self.m44],
        ]
    }

    pub fn to_array_switched(&self) -> [[f32; 4]; 4] {
        [
            [self.m11, self.m21, self.m31, self.m41],
            [self.m12, self.m22, self.m32, self.m42],
            [self.m13, self.m23, self.m33, self.m43],
            [self.m14, self.m24, self.m34, self.m44],
        ]
    }
    
}

impl<T> Add for Matrix4<T> where T:Add<Output=T> + Copy {
    type Output=Matrix4<T>;
    fn add(self, rhs:Self) -> Self::Output {
        Matrix4 {
            m11: self.m11 + rhs.m11, m12: self.m12 + rhs.m12,
            m13: self.m13 + rhs.m13, m14: self.m14 + rhs.m14,
            m21: self.m21 + rhs.m21, m22: self.m22 + rhs.m22,
            m23: self.m23 + rhs.m23, m24: self.m24 + rhs.m24,
            m31: self.m31 + rhs.m31, m32: self.m32 + rhs.m32,
            m33: self.m33 + rhs.m33, m34: self.m34 + rhs.m34,
            m41: self.m41 + rhs.m41, m42: self.m42 + rhs.m42,
            m43: self.m43 + rhs.m43, m44: self.m44 + rhs.m44,
        } 
    }
}

impl<T> Mul for Matrix4<T> where T:Add<Output=T> + Mul<Output=T> + Copy {
    type Output=Matrix4<T>;
    fn mul(self, rhs:Self) -> Self::Output {
        Matrix4 {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31 + self.m14 * rhs.m41,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32 + self.m14 * rhs.m42,
            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33 + self.m14 * rhs.m43,
            m14: self.m11 * rhs.m14 + self.m12 * rhs.m24 + self.m13 * rhs.m34 + self.m14 * rhs.m44,
            
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31 + self.m24 * rhs.m41,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32 + self.m24 * rhs.m42,
            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33 + self.m24 * rhs.m43,
            m24: self.m21 * rhs.m14 + self.m22 * rhs.m24 + self.m23 * rhs.m34 + self.m24 * rhs.m44,

            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31 + self.m34 * rhs.m41,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32 + self.m34 * rhs.m42,
            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33 + self.m34 * rhs.m43,
            m34: self.m31 * rhs.m14 + self.m32 * rhs.m24 + self.m33 * rhs.m34 + self.m34 * rhs.m44,

            m41: self.m41 * rhs.m11 + self.m42 * rhs.m21 + self.m43 * rhs.m31 + self.m44 * rhs.m41,
            m42: self.m41 * rhs.m12 + self.m42 * rhs.m22 + self.m43 * rhs.m32 + self.m44 * rhs.m42,
            m43: self.m41 * rhs.m13 + self.m42 * rhs.m23 + self.m43 * rhs.m33 + self.m44 * rhs.m43,
            m44: self.m41 * rhs.m14 + self.m42 * rhs.m24 + self.m43 * rhs.m34 + self.m44 * rhs.m44,
        }
    }
}

impl<T> MulAssign for Matrix4<T> where T:Add<Output=T> + Mul<Output=T> + Copy {
    fn mul_assign(&mut self, rhs:Self) {
        let m11 = self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31 + self.m14 * rhs.m41;
        let m12 = self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32 + self.m14 * rhs.m42;
        let m13 = self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33 + self.m14 * rhs.m43;
        let m14 = self.m11 * rhs.m14 + self.m12 * rhs.m24 + self.m13 * rhs.m34 + self.m14 * rhs.m44;
        let m21 = self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31 + self.m24 * rhs.m41;
        let m22 = self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32 + self.m24 * rhs.m42;
        let m23 = self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33 + self.m24 * rhs.m43;
        let m24 = self.m21 * rhs.m14 + self.m22 * rhs.m24 + self.m23 * rhs.m34 + self.m24 * rhs.m44;
        let m31 = self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31 + self.m34 * rhs.m41;
        let m32 = self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32 + self.m34 * rhs.m42;
        let m33 = self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33 + self.m34 * rhs.m43;
        let m34 = self.m31 * rhs.m14 + self.m32 * rhs.m24 + self.m33 * rhs.m34 + self.m34 * rhs.m44;
        let m41 = self.m41 * rhs.m11 + self.m42 * rhs.m21 + self.m43 * rhs.m31 + self.m44 * rhs.m41;
        let m42 = self.m41 * rhs.m12 + self.m42 * rhs.m22 + self.m43 * rhs.m32 + self.m44 * rhs.m42;
        let m43 = self.m41 * rhs.m13 + self.m42 * rhs.m23 + self.m43 * rhs.m33 + self.m44 * rhs.m43;
        let m44 = self.m41 * rhs.m14 + self.m42 * rhs.m24 + self.m43 * rhs.m34 + self.m44 * rhs.m44;

        self.m11 = m11;
        self.m12 = m12;
        self.m13 = m13;
        self.m14 = m14;
        self.m21 = m21;
        self.m22 = m22;
        self.m23 = m23;
        self.m24 = m24;
        self.m31 = m31;
        self.m32 = m32;
        self.m33 = m33;
        self.m34 = m34;
        self.m41 = m41;
        self.m42 = m42;
        self.m43 = m43;
        self.m44 = m44;
    }
}

// erzeugt eine 4x4 Matrix aus einem 2d array (column major)
impl<T> From<[[T; 4]; 4]> for Matrix4<T> where T:Copy {
    fn from(array: [[T; 4]; 4]) -> Self {
        Matrix4 {
            m11: array[0][0], m12: array[0][1],
            m13: array[0][2], m14: array[0][3],
            m21: array[1][0], m22: array[1][1],
            m23: array[1][2], m24: array[1][3],
            m31: array[2][0], m32: array[2][1],
            m33: array[2][2], m34: array[2][3],
            m41: array[3][0], m42: array[3][1],
            m43: array[3][2], m44: array[3][3],
        }
    }
}

// umkehroperation zu from
impl<T> Into<[[T; 4]; 4]> for Matrix4<T> where T:Copy {
    fn into(self) -> [[T; 4]; 4] {
        [
            [self.m11, self.m12, self.m13, self.m14],
            [self.m21, self.m22, self.m23, self.m24],
            [self.m31, self.m32, self.m33, self.m34],
            [self.m41, self.m42, self.m43, self.m44],
        ]
    }
}

pub fn identity() -> Matrix4<f32> {
    Matrix4::<f32>::from([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}
