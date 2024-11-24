// https://doc.rust-lang.org/rust-by-example/std/panic.html

#[cfg(test)]
mod tests {
    use std::panic;

    #[test]
    fn test_panic() {
        fn div(x: i32, y: i32) -> i32 {
            if y == 0 {
                panic!("div by zero");
            } else {
                x / y
            }
        }

        let result = div(9, 3);
        assert_eq!(result, 3);

        let result = panic::catch_unwind(|| {
            div(1, 0);
        });
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().downcast_ref::<&str>(),
            Some(&"div by zero")
        );

        let result = panic::catch_unwind(|| div(4, 2));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }
}
