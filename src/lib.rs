mod error;
mod fieldelement;

use error::{PointError};
use std::fmt::Display;
use std::ops::Mul;
use std::cmp::{PartialEq, Eq};

// Type that represents a Big Integer stored as a Vector of u32
struct U256(Vec<u32>);

// How many hex digits should our own Big Integer digit contain
const HEX_SIZE: u32 = 8;
// We make radix 10**9, so each "digit" of the long number contains 9 decimal
// digits at once
const RADIX: u32 = 0xffff_ffff;

impl U256 {
    fn new(number: &str) -> Self {
        // Create an iterator over the String we got
        let decimal_vec = number.chars().collect::<Vec<char>>();

        // Parse the number in a Vec of u32
        let bigint: Vec<u32> = decimal_vec.chunks(8usize)
            .map(|chunks| chunks.iter().rev().enumerate()
                .fold(0u32, |n, (i, item)| {
                println!("{} {}", n, i);
                n as u32 + 16u32.pow(i as u32) * item.to_digit(16).unwrap() }))
            .rev().collect();
        U256(bigint)
    }
}

/*
pub trait Pow<Rhs> {
    type Output;
    fn pow(self, rhs: Rhs) -> Self::Output;
}

/// Represents a `point` on an eliptic curve for the form
/// y**2 = x**3 + 5*x + 7, symmetric over the x-axis
#[derive(Copy, Clone, Debug)]
pub struct Point<T> {
    /// Value for the x-axis
    x: T,
    /// Value for the y-axis
    y: T,
    /// Curve specific number
    a: T,
    /// Curve specific number
    b: T,
}

impl<T: Pow<T> + Display + Mul<Output=T>> Point<T> {
    fn new(x: T, y: T, a: T, b: T) -> Result<Self, PointError> {
        if y.pow(2 as T) != x.pow(3 as T) + a * x + b {
            return Err(PointError {
                message: format!("({}, {}) is not on the curve", x, y)
            })
        }

        Ok(Point {
            x, y, a, b
        })

    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
                && self.a == other.a && self.b == other.b
    }
}

impl<T: Eq> Eq for Point<T> {}
*/



#[cfg(test)]
mod tests {
    use super::*;
    use crate::fieldelement::{FieldElement};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn fieldelement_new() {
        let new_elem = FieldElement::new(10, 13);
        assert!(new_elem.is_ok());

        let new_elem = FieldElement::new(20, 13);
        assert!(new_elem.is_err());
    }

    #[test]
    #[should_panic]
    fn fieldelement_new_panic() {
        let new_elem = FieldElement::new(20, 13);
        let num = new_elem.unwrap();
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

    #[test]
    fn u256_test() {
        let a = U256::new("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    }

    /*
    #[test]
    fn point_new() {
        let p1 = Point::new(-1, -1, 5, 7);
    }
    */
}
