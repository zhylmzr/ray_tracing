#![allow(dead_code)]
use crate::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        return self.orig + t * self.dir;
    }
    pub fn origin(&self) -> Vec3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        assert_eq!(
            Ray {
                orig: Vec3(1.0, 1.0, 1.0),
                dir: Vec3(1.0, 1.0, 1.0),
            },
            Ray {
                orig: Vec3(1.0, 1.0, 1.0),
                dir: Vec3(1.0, 1.0, 1.0),
            }
        );
    }
}
