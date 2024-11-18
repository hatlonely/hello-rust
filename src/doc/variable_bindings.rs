#[cfg(test)]
mod tests {
    #[test]
    fn mutability() {
        let mut x = 5;
        println!("x: {}", x);
        x = 10;
        println!("x: {}", x);
    }

    #[test]
    fn scope() {
        let x = 5;
        {
            let y = 10;
            println!("x: {}", x);
            println!("y: {}", y);
        }
        // println!("y: {}", y); // error: cannot find value `y` in this scope
    }

    #[test]
    fn shadowing() {
        let x = 5;
        println!("x: {}", x);

        {
            let x = "hello";
            println!("x: {}", x);
        }
    }

    #[test]
    fn declare() {
        let x;
        // println!("x: {}", x); // error: use of possibly-uninitialized `x`
        x = 5;
        println!("x: {}", x);

        let y;
        {
            y = 10;
            println!("y: {}", y);
        }
        println!("y: {}", y);
    }

    #[test]
    fn binding() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("x: {}", x);

        let spaces = "   ";
        let spaces = spaces.len();
        println!("spaces: {}", spaces);
    }

    #[test]
    fn freeze() {
        let mut x = 5;
        let y = x;
        x = 10;
        println!("x: {}, y: {}", x, y);
    }
}
