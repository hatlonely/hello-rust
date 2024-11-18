// https://doc.rust-lang.org/rust-by-example/conversion.html

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

        let even2: Result<EvenNumber, ()> = 6.try_into();
        println!("even2: {:?}", even2);

        assert_eq!(even2, Ok(EvenNumber(6)));
    }

    #[test]
    fn convert_to_string() {
        use std::fmt;
        use std::str::FromStr;

        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        let point = Point { x: 3, y: 4 };
        println!("point: {}", point);

        impl FromStr for Point {
            type Err = std::num::ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let coords: Vec<&str> = s
                    .trim_matches(|p| p == '(' || p == ')')
                    .split(',')
                    .collect();

                let x_fromstr = coords[0].parse::<i32>()?;
                let y_fromstr = coords[1].parse::<i32>()?;

                Ok(Point {
                    x: x_fromstr,
                    y: y_fromstr,
                })
            }
        }

        let p = Point::from_str("(1,2)");
        println!("p: {:?}", p);
    }
}
