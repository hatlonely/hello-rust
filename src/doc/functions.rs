// https://doc.rust-lang.org/rust-by-example/fn.html

#[cfg(test)]
mod tests {
    #[test]
    fn functions() {
        fn fab(n: u32) -> u32 {
            if n == 0 {
                return 0;
            } else if n == 1 {
                return 1;
            } else {
                return fab(n - 1) + fab(n - 2);
            }
        }

        println!("fab(0): {}", fab(0));
    }

    #[test]
    fn associated_functions_methods() {
        struct Circle {
            radius: f64,
        }

        impl Circle {
            fn new(radius: f64) -> Circle {
                Circle { radius }
            }

            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }

        let c = Circle::new(2.0);
        println!("c.area(): {}", c.area());
    }

    #[test]
    fn closures() {
        let plus_one = |x: i32| x + 1;
        println!("plus_one(1): {}", plus_one(1));

        let plus_two = |x| {
            let mut result: i32 = x;
            result += 1;
            result += 1;
            result
        };
        println!("plus_two(1): {}", plus_two(1));

        let num = 5;
        let plus_num = |x: i32| x + num;
        println!("plus_num(1): {}", plus_num(1));
    }

    #[test]
    fn closures_capturing() {
        let num = 5;
        let plus_num = |x: i32| x + num;
        println!("plus_num(1): {}", plus_num(1));

        let mut num = 5;
        {
            let plus_num = |x: i32| x + num;
            println!("plus_num(1): {}", plus_num(1));
        }

        let plus_num = |x: i32, y: i32| x + y;
        println!("plus_num(1, 2): {}", plus_num(1, 2));
    }
}
