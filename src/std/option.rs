// https://doc.rust-lang.org/rust-by-example/std/option.html

#[cfg(test)]
mod tests {
    #[test]
    fn test_option() {
        let x = 3.0;
        let y = 2.0;

        // Option is a type that represents the presence or absence of a value.
        let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };

        match result {
            Some(z) => println!("{}/{} = {}", x, y, z),
            None => println!("Cannot divide {} by {}", x, y),
        }

        // If let is a less verbose way to handle values that match one pattern.
        if let Some(z) = result {
            println!("z = {}", z);
        }
    }
}
