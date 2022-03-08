mod error;

use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::convert::TryFrom;
use error::{FieldError};

/// Represents a single finite field element
#[derive(Debug, Clone, Copy)]
struct FieldElement {
    /// Elements numerical value
    num: u32,
    /// Prime number representing the order(or size) of the set
    order: u32,
}

impl FieldElement {
    /// Creates a new FieldElement instance
    fn new(num: u32, order: u32) -> Result<Self, FieldError> {
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

    fn pow(&self, exponent: i32) -> FieldElement {
        let mut exp: u32 = 0;
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn fieldelement_new() {
        let new_elem = FieldElement::new(10, 13);
        assert!(new_elem.is_ok());

        assert_eq!(10, new_elem.unwrap().num);

        let new_elem = FieldElement::new(20, 13);
        assert!(new_elem.is_err());
    }

    #[test]
    #[should_panic]
    fn fieldelement_new_panic() {
        let new_elem = FieldElement::new(20, 13);
        let num = new_elem.unwrap().num;
        println!("{}", num);
    }

    #[test]
    fn fieldelement_eq() {
        let a = FieldElement::new(7, 13).unwrap();
        let b = FieldElement::new(6, 13).unwrap();
        assert_eq!(a, a);
        assert_ne!(a, b);
        println!("{}", -27 % 13 + 13);
    }

    #[test]
    fn fieldelement_add() {
        let a = FieldElement::new(7, 13).unwrap();
        let b = FieldElement::new(12, 13).unwrap();
        let c = FieldElement::new(6, 13).unwrap();
        assert_eq!((a+b), c);
        assert_ne!((a+c), b);

        let a = FieldElement::new(17, 57).unwrap();
        let b = FieldElement::new(42, 57).unwrap();
        let c = FieldElement::new(49, 57).unwrap();
        let d = FieldElement::new(51, 57).unwrap();
        assert_eq!(a+b+c, d);
    }

    #[test]
    #[should_panic]
    fn fieldelement_add_panic() {
        let a = FieldElement::new(7, 13).unwrap();
        let b = FieldElement::new(10, 19).unwrap();
        let c = FieldElement::new(6, 13).unwrap();
        assert_eq!((a+b), c);
    }

    #[test]
    fn fieldelement_sub() {
        let a = FieldElement::new(9, 57).unwrap();
        let b = FieldElement::new(29, 57).unwrap();
        let c = FieldElement::new(37, 57).unwrap();
        assert_eq!(a-b, c);
        println!("{}", a-b);

        let a = FieldElement::new(52, 57).unwrap();
        let b = FieldElement::new(30, 57).unwrap();
        let c = FieldElement::new(38, 57).unwrap();
        let d = FieldElement::new(41, 57).unwrap();
        assert_eq!(a-b-c, d)
    }

    #[test]
    #[should_panic]
    fn fieldelement_sub_panic() {
        let a = FieldElement::new(9, 57).unwrap();
        let b = FieldElement::new(29, 46).unwrap();
        let c = FieldElement::new(37, 57).unwrap();
        assert_eq!((a-b-c), b);
    }

    #[test]
    fn fieldelement_mul() {
        let a = FieldElement::new(3, 13).unwrap();
        let b = FieldElement::new(12, 13).unwrap();
        let c = FieldElement::new(10, 13).unwrap();
        assert_eq!(a*b, c);

        let a = FieldElement::new(95, 97).unwrap();
        let b = FieldElement::new(45, 97).unwrap();
        let c = FieldElement::new(31, 97).unwrap();
        let d = FieldElement::new(23, 97).unwrap();
        assert_eq!(a*b*c, d);
    }

    #[test]
    #[should_panic]
    fn fieldelement_mul_panic() {
        let a = FieldElement::new(3, 13).unwrap();
        let b = FieldElement::new(12, 10).unwrap();
        let c = FieldElement::new(10, 13).unwrap();
        assert_eq!(a*b, c);

        let a = FieldElement::new(95, 97).unwrap();
        let b = FieldElement::new(45, 97).unwrap();
        let c = FieldElement::new(31, 87).unwrap();
        let d = FieldElement::new(23, 97).unwrap();
        assert_eq!(a*b*c, d);
    }

    #[test]
    fn fieldelement_pow() {
        let a = FieldElement::new(3, 13).unwrap();
        let b = 3;
        let c = FieldElement::new(1, 13).unwrap();
        assert_eq!(a.pow(b), c);

        let a = FieldElement::new(2, 19).unwrap();
        let b = FieldElement::new(7, 19).unwrap();
        let b = b.pow(-1);
        let c = FieldElement::new(3, 19).unwrap();
        assert_eq!(a*b, c);

        let a = FieldElement::new(2, 19).unwrap();
        let b = FieldElement::new(7, 19).unwrap();
        let b = b.pow(-2);
        let c = FieldElement::new(14, 19).unwrap();
        assert_eq!(a*b, c);

        let a = FieldElement::new(7, 19).unwrap();
        let b = FieldElement::new(5, 19).unwrap();
        let b = b.pow(-1);
        let c = FieldElement::new(9, 19).unwrap();
        assert_eq!(a*b, c);

        let a = FieldElement::new(11, 31).unwrap();
        let b = FieldElement::new(4, 31).unwrap();
        let b = b.pow(-4);
        let c = FieldElement::new(13, 31).unwrap();
        assert_eq!(a*b, c);
    }

    #[test]
    fn fieldelement_div() {
        let a = FieldElement::new(3, 31).unwrap();
        let b = FieldElement::new(24, 31).unwrap();
        let c = FieldElement::new(4, 31).unwrap();
        assert_eq!(a/b, c);

        let a = FieldElement::new(2, 19).unwrap();
        let b = FieldElement::new(7, 19).unwrap();
        let c = FieldElement::new(3, 19).unwrap();
        assert_eq!(a/b, c);

        let a = FieldElement::new(7, 19).unwrap();
        let b = FieldElement::new(5, 19).unwrap();
        let c = FieldElement::new(9, 19).unwrap();
        assert_eq!(a/b, c);
    }

}
