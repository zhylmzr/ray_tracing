#![allow(dead_code)]
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new_unit_vector() -> Self {
        let unit = (1.0 / 3.0 as f64).sqrt();
        Self(unit, unit, unit)
    }

    pub fn x(self) -> f64 {
        self.0
    }
    pub fn y(self) -> f64 {
        self.1
    }
    pub fn z(self) -> f64 {
        self.2
    }
    pub fn r(self) -> f64 {
        self.0
    }
    pub fn g(self) -> f64 {
        self.1
    }
    pub fn b(self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }
    pub fn squared_length(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn dot(self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
    pub fn cross(self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            -(self.0 * rhs.2 - self.2 * rhs.0),
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
    pub fn unit_vector(self) -> Self {
        let len = self.length();
        self / len
    }
    pub fn gen_colors(&self) -> Vec<u8> {
        let k = 255.999;
        format!(
            "{} {} {}\n",
            (k * self.0) as i32,
            (k * self.1) as i32,
            (k * self.2) as i32
        )
        .into()
    }
}

macro_rules! create_ops {
    ($($trait:ident $func:ident), *) => {
        $(impl $trait<Vec3> for Vec3 {
            type Output = Vec3;
            fn $func(self, rhs: Self) -> Self::Output {
                Self($trait::$func(self.0, rhs.0), $trait::$func(self.1, rhs.1), $trait::$func(self.2, rhs.2))
            }
        })*

        $(impl $trait<Vec3> for f64 {
            type Output = Vec3;
            fn $func(self, rhs: Vec3) -> Self::Output {
                Vec3($trait::$func(self, rhs.0), $trait::$func(self, rhs.1), $trait::$func(self, rhs.2))
            }
        })*

        $(impl $trait<f64> for Vec3 {
            type Output = Vec3;
            fn $func(self, rhs: f64) -> Self::Output {
                Self($trait::$func(self.0, rhs), $trait::$func(self.1, rhs), $trait::$func(self.2, rhs))
            }
        })*
    };
}

macro_rules! create_ops_mut {
    ($($trait:ident $func:ident), *) => {
        $(impl $trait<Vec3> for Vec3 {
            fn $func(&mut self, rhs: Self) {
                $trait::$func(&mut self.0, rhs.0);
                $trait::$func(&mut self.1, rhs.1);
                $trait::$func(&mut self.2, rhs.2);
            }
        })*

        $(impl $trait<f64> for Vec3 {
            fn $func(&mut self, rhs: f64) {
                $trait::$func(&mut self.0, rhs);
                $trait::$func(&mut self.1, rhs);
                $trait::$func(&mut self.2, rhs);
            }
        })*
    };
}

create_ops!(Add add, Sub sub, Mul mul, Div div);
create_ops_mut!(AddAssign add_assign, SubAssign sub_assign, MulAssign mul_assign, DivAssign div_assign);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3() {
        let mut a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(4.0, 5.0, 2.0);

        a += b;
        assert_eq!(Vec3(5.0, 7.0, 5.0), a);

        a = a.unit_vector();
        assert_eq!(Vec3::new_unit_vector().length(), a.length());
    }
}
