mod error;
mod fieldelement;


#[cfg(test)]
mod tests {
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

}
