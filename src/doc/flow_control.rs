#[cfg(test)]
mod tests {
    #[test]
    fn if_else() {
        let n = 5;
        if n < 0 {
            println!("{} is negative", n);
        } else if n > 0 {
            println!("{} is positive", n);
        } else {
            println!("{} is zero", n);
        }
    }

    #[test]
    #[allow(unreachable_code)]
    #[allow(unused_labels)]
    fn loop_while() {
        let mut n = 0;
        loop {
            n += 1;
            if n == 3 {
                continue;
            }
            if n > 5 {
                break;
            }
            println!("n: {}", n);
        }

        'outer: loop {
            println!("Entered the outer loop");
            'inner: loop {
                println!("Entered the inner loop");
                break 'outer;
            }
            println!("This point will never be reached");
        }

        let mut n = 0;
        while n < 5 {
            n += 1;
            println!("n: {}", n);
        }
    }

    #[test]
    fn for_range() {
        for n in 1..=5 {
            println!("n: {}", n);
        }

        for i in vec![1, 2, 3].iter() {
            println!("i: {}", i);
        }
    }

    #[test]
    fn match_pattern() {
        let number = 13;
        match number {
            1 => println!("One"),
            2 | 3 | 5 | 7 | 11 => println!("Prime"),
            13..=19 => println!("Teen"),
            _ => println!("Ain't special"),
        }

        let boolean = true;
        let binary = match boolean {
            false => 0,
            true => 1,
        };
        println!("{} -> {}", boolean, binary);
    }
}
