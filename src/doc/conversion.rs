#[cfg(test)]
mod tests {

    #[test]
    fn test_from() {
        use std::convert::From;

        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl From<i32> for Number {
            fn from(item: i32) -> Self {
                Number { value: item }
            }
        }

        let num1 = Number::from(30);
        println!("num1: {:?}", num1);
        println!("num1.value: {:?}", num1.value);

        // into 能自动调用 from，反过来不行
        let num2: Number = 40.into();
        println!("num2: {:?}", num2);
        println!("num2.value: {:?}", num2.value);
    }

    #[test]
    fn test_into() {
        use std::convert::Into;

        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl Into<Number> for i32 {
            fn into(self) -> Number {
                Number { value: self }
            }
        }

        let num2: Number = 40.into();
        println!("num2: {:?}", num2);
        println!("num2.value: {:?}", num2.value);
    }

    #[test]
    fn test_try_from() {
        use std::convert::TryFrom;

        #[derive(Debug, PartialEq)]
        struct EvenNumber(i32);

        impl TryFrom<i32> for EvenNumber {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value % 2 == 0 {
                    Ok(EvenNumber(value))
                } else {
                    Err(())
                }
            }
        }

        let even = EvenNumber::try_from(8);
        println!("even: {:?}", even);
        let odd = EvenNumber::try_from(5);
        println!("odd: {:?}", odd);

        assert_eq!(even, Ok(EvenNumber(8)));
        assert_eq!(odd, Err(()));
    }
}
