use std::ops::*;
use std::fmt::Write;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(a: f64, b: f64, c: f64) -> Self {
        Self {
            x: a,
            y: b,
            z: c
        }
    }

    fn write_into<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write(&self.to_string().into_bytes())?;
        Ok(())
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn r(&self) -> f64 {
        self.x
    }

    fn g(&self) -> f64 {
        self.y
    }

    fn b(&self) -> f64 {
        self.z
    }

    fn dot(&self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z 
    }

    fn cross(&self, v: &Self) -> Self {
        Self {
            x: self.y * v.z - self.z * v.y,  
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()

    }

    fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    fn unit_vector(&self) -> Self {
        *self / self.length()

    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index {} out of bounds. Len is {}", idx, 3)
        }
    }
} 

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}


impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl From<f64> for Vec3 {
    fn from(value: f64) -> Self {
        Self {x: value, y: value, z: value}
    }
}

#[macro_export]
macro_rules! vec3 {
    ($val:literal) => {
        Vec3::new($val as f64, $val as f64, $val as f64)
    };
    ($x:literal,$y:literal,$z:literal) => {
        Vec3::new($x as f64, $y as f64, $z as f64)
    };
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn additions() {
        let v1 = vec3![1,2,3];
        let v2 = vec3![1];
        //add
        let mut v3 = v1 + v2;
        assert_eq!(v3, vec3![2, 3, 4]);
        //add float
        assert_eq!(v3 + 2.0, vec3![4, 5, 6]);
        //add assign
        v3 += v2;
        assert_eq!(v3, vec3![3, 4, 5]);
        //add assign float
        v3 += 1.0;
        assert_eq!(v3, vec3![4, 5, 6]);
    }

    #[test]
    fn subtractions() {
        let v1 = vec3![4,5,6];
        let v2 = vec3![1];
        //sub
        let mut v3 = v1 - v2;
        assert_eq!(v3, vec3![3, 4, 5]);
        //sub float
        assert_eq!(v3 - 2.0, vec3![1, 2, 3]);
        //sub assign
        v3 -= v2;
        assert_eq!(v3, vec3![2, 3, 4]);
        //sub assign float
        v3 -= 1.0;
        assert_eq!(v3, vec3![1,2,3]);
    }

    #[test]
    fn multiplications() {
        let v1 = vec3![2,4,8];
        let v2 = vec3![2];
        let t = 4.0;
        //mul
        let mut v3 = v1 * v2;
        assert_eq!(v3, vec3![4, 8, 16]);
        //mul float
        assert_eq!(v3 * t, vec3![16, 32, 64]);
        //mul assign
        v3 *= v2;
        assert_eq!(v3, vec3![8, 16, 32]);
        //mul assign float
        v3 *= 10.0;
        assert_eq!(v3, vec3![80, 160, 320]);
    }

    #[test]
    fn divisions() {
        let v1 = vec3![4,8,16];
        let v2 = vec3![2];
        let t = 4.0;
        //div
        let mut v3 = v1 / v2;
        assert_eq!(v3, vec3![2, 4, 8]);
        //div float
        assert_eq!(v1 / t, vec3![1, 2, 4]);
        //div assign
        v3 /= v2;
        assert_eq!(v3, vec3![1, 2, 4]);
        //div assign float
        v3 /= 2.0;
        assert_eq!(v3, vec3![0.5, 1, 2]);
    }

    #[test]
    fn negations() {
        let v1 = vec3![2,3,4];
        assert_eq!(-v1, vec3![-2, -3, -4]);
    }

    #[test]
    fn indexing() {
        let v1 = vec3![2,3,4];
        assert_eq!(v1[0], 2.0);
        assert_eq!(v1[1], 3.0);
        assert_eq!(v1[2], 4.0);
    }

    #[test]
    fn dot_product() {
        let v1 = vec3![1,2,3];
        let v2 = vec3![4,5,6];
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn cross_product() {
        let v1 = vec3![1,2,3];
        let v2 = vec3![4,5,6];
        assert_eq!(v1.cross(&v2), vec3![-3, 6, -3]);
    }

    #[test]
    fn length() {
        let v1 = vec3![3,4,12];
        assert_eq!(v1.length(), 13.0);
    }

    #[test]
    fn unit() {
        let v1 = vec3![3,4,12];
        let uv = v1.unit_vector();
        assert_eq!(uv.length(), 1.0);
    }
}
