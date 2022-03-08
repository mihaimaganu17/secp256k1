use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::convert::TryFrom;
use crate::error::{FieldError};

/// Represents a single finite field element
#[derive(Debug, Clone, Copy)]
pub struct FieldElement {
    /// Elements numerical value
    num: u32,
    /// Prime number representing the order(or size) of the set
    order: u32,
}

impl FieldElement {
    /// Creates a new FieldElement instance
    pub fn new(num: u32, order: u32) -> Result<Self, FieldError> {
        if num >= order {
            return Err(FieldError {
                message: format!("Number {} not in field range 0 and {}",
                             num, order-1)
            })
        }
        Ok(FieldElement {
            num,
            order
        })
    }

    pub fn pow(&self, exponent: i32) -> FieldElement {
        // Placeholder for converting exponent in case it is negative
        let mut exp: u32 = 0;

        // Reduce high exponents
        let exponent = exponent % (i32::try_from(self.order).unwrap() - 1);
        if exponent < 0 {
            exp = (i32::try_from(self.order).unwrap() - 1 + exponent)
                .try_into().unwrap();
        } else {
            exp = exponent.try_into().unwrap();
        }

        let mut res = 1;
        let mut num = self.num;
        while exp > 0 {
            if exp & 1 != 0 {
                res = (res * num) % self.order;
            }
            num = (num * num) % self.order;
            exp >>= 1;
        }


        FieldElement {
            num: res,
            order: self.order
        }
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.num, self.order)
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if self.order != other.order {
            panic!("Field elements {} and {} do not have the \
                    same order prime", self, other)
        } else {
            Self {
                num: (self.num + other.num) % self.order,
                order: self.order
            }
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        if self.order != other.order {
            panic!("Field elements {} and {} do not have the \
                same order prime", self, other)
        } else {
            let num = (self.num + self.order  - other.num) % self.order;

            Self {
                num,
                order: self.order
            }
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self.order != other.order {
            panic!("Field elements {} and {} do not have the \
                same order prime", self, other)
        } else {
            let num = (self.num * other.num) % self.order;
            Self {
                num,
                order: self.order
            }
        }

    }
}

impl Div for FieldElement {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let other = other.pow(-1);
        self * other
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.order == self.order
    }
}

impl Eq for FieldElement {}
