use std::cmp::Ordering;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

pub struct OperatorPointTwinNormal {
    pub x: i32,
    pub y: i32,
}

impl Add for OperatorPointTwinNormal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl PartialEq for OperatorPointTwinNormal {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl PartialOrd for OperatorPointTwinNormal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

pub struct OperatorValueTwinNormal {
    pub value: i32,
}

impl Add for OperatorValueTwinNormal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}
impl Sub for OperatorValueTwinNormal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}
impl Mul for OperatorValueTwinNormal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}
impl Div for OperatorValueTwinNormal {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value / rhs.value,
        }
    }
}
impl Rem for OperatorValueTwinNormal {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value % rhs.value,
        }
    }
}
impl Neg for OperatorValueTwinNormal {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { value: -self.value }
    }
}
impl Not for OperatorValueTwinNormal {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self { value: !self.value }
    }
}
impl BitAnd for OperatorValueTwinNormal {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
    }
}
impl BitOr for OperatorValueTwinNormal {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}
impl BitXor for OperatorValueTwinNormal {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value ^ rhs.value,
        }
    }
}
impl Shl<u32> for OperatorValueTwinNormal {
    type Output = Self;
    fn shl(self, rhs: u32) -> Self::Output {
        Self {
            value: self.value << rhs,
        }
    }
}
impl Shr<u32> for OperatorValueTwinNormal {
    type Output = Self;
    fn shr(self, rhs: u32) -> Self::Output {
        Self {
            value: self.value >> rhs,
        }
    }
}
