// https://doc.rust-lang.org/rust-by-example/std/result.html

#[cfg(test)]
mod tests {
    #[test]
    fn test_result() {
        fn div(x: f64, y: f64) -> Result<f64, &'static str> {
            if y == 0.0 {
                Err("division by zero")
            } else {
                Ok(x / y)
            }
        }

        let res = div(5.0, 2.0);
        match res {
            Ok(v) => println!("Result: {}", v),
            Err(e) => println!("Error: {}", e),
        }

        let res = div(5.0, 0.0);
        match res {
            Ok(v) => println!("Result: {}", v),
            Err(e) => println!("Error: {}", e),
        }

        fn sqrt(x: f64) -> Result<f64, &'static str> {
            if x < 0.0 {
                Err("negative float")
            } else {
                Ok(x.sqrt())
            }
        }

        fn op(x: f64, y: f64) -> Result<f64, &'static str> {
            let ratio = div(x, y)?;
            sqrt(ratio)
        }

        let res = op(5.0, 2.0);
        match res {
            Ok(v) => println!("Result: {}", v),
            Err(e) => println!("Error: {}", e),
        }
    }
}
