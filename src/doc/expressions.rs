// https://doc.rust-lang.org/rust-by-example/expression.html

#[cfg(test)]
mod tests {
    #[test]
    fn expressions() {
        let x = 5;
        let y = {
            let x_squared = x * x;
            let x_cube = x_squared * x;
            x_cube + x_squared + x
        };

        println!("x: {}", x);
        println!("y: {}", y);

        assert_eq!(x, 5);
        assert_eq!(y, 155);
    }
}
