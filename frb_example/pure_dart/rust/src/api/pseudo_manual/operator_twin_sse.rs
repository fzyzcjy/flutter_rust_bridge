// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `operator.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use std::cmp::Ordering;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

pub struct OperatorPointTwinSse {
    pub x: i32,
    pub y: i32,
}

impl Add for OperatorPointTwinSse {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl PartialEq for OperatorPointTwinSse {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl PartialOrd for OperatorPointTwinSse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

pub struct OperatorValueTwinSse {
    pub value: i32,
}

impl Add for OperatorValueTwinSse {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}
impl Sub for OperatorValueTwinSse {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}
impl Mul for OperatorValueTwinSse {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}
impl Div for OperatorValueTwinSse {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value / rhs.value,
        }
    }
}
impl Rem for OperatorValueTwinSse {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value % rhs.value,
        }
    }
}
impl Neg for OperatorValueTwinSse {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { value: -self.value }
    }
}
impl Not for OperatorValueTwinSse {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self { value: !self.value }
    }
}
impl BitAnd for OperatorValueTwinSse {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
    }
}
impl BitOr for OperatorValueTwinSse {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}
impl BitXor for OperatorValueTwinSse {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value ^ rhs.value,
        }
    }
}
impl Shl<u32> for OperatorValueTwinSse {
    type Output = Self;
    fn shl(self, rhs: u32) -> Self::Output {
        Self {
            value: self.value << rhs,
        }
    }
}
impl Shr<u32> for OperatorValueTwinSse {
    type Output = Self;
    fn shr(self, rhs: u32) -> Self::Output {
        Self {
            value: self.value >> rhs,
        }
    }
}
