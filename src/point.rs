use core::ops::{Add, Mul};
use num::BigInt;

#[derive(Debug, Clone)]
pub struct Point {
    x: BigInt,
    y: BigInt,
}

impl Point {
    pub fn new(x: BigInt, y: BigInt) -> Self {
        Self { x, y }
    }

    pub fn neg(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: -self.y.clone(),
        }
    }

    pub fn mul(&self, k: BigInt) -> Self {
        let mut base = self.clone();
        let mut result: Option<Self> = None;

        for c in k.to_str_radix(2).chars() {
            base = base.clone() + self.clone();
            if c == '1' {
                if result.clone().is_none() {
                    result = Some(base.clone());
                } else {
                    result = Some(result.clone().unwrap() + base.clone())
                }
            }
        }

        result.unwrap()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let p = BigInt::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        );
        let one = BigInt::parse_bytes(b"1", 10).unwrap();
        if self.x == rhs.x && self.y == rhs.y {
            let top: BigInt = self.x.pow(2).mul(3) / (self.y.clone().mul(2));
            let s = top.modpow(&one, &p.clone().as_ref().unwrap());
            let x_new = (s.pow(2) - rhs.x - self.x.clone()).modpow(&one, &p.clone().unwrap());
            let y_new = (s * (self.x - x_new.clone()) - self.y).modpow(&one, &p.as_ref().unwrap());
            Self { x: x_new, y: y_new }
        } else {
            let top: BigInt = (rhs.y - self.y.clone()) / (rhs.x.clone() - self.x.clone());
            let s = top.modpow(&one, &p.clone().as_ref().unwrap());
            let x_new = (s.pow(2) - rhs.x - self.x.clone()).modpow(&one, &p.clone().unwrap());
            let y_new = (s * (self.x - x_new.clone()) - self.y).modpow(&one, &p.as_ref().unwrap());
            Self { x: x_new, y: y_new }
        }
    }
}
