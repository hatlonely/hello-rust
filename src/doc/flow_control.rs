// https://doc.rust-lang.org/rust-by-example/flow_control.html

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

    #[test]
    fn match_destructing_tuples() {
        let triple = (0, -2, 3);
        match triple {
            (0, y, z) => println!("First is `0`, `y` is `{}`, and `z` is `{}`", y, z),
            (1, ..) => println!("First is `1` and the rest doesn't matter"),
            _ => println!("It doesn't matter what they are"),
        }
    }

    #[test]
    fn match_destructing_arrays() {
        let numbers = [1, 2, 3];
        match numbers {
            [first, .., last] => {
                println!("Some numbers: {}, {}, {}", first, numbers[1], last);
            }
        }
    }

    #[test]
    fn match_destructing_slices() {
        let slice = &[1, 2, 3, 4, 5];
        match slice {
            [1, a, b, ..] => println!("{}, {}", a, b),
            [2, a, .., b] => println!("{}, .., {}", a, b),
            _ => println!("Unknown"),
        }
    }

    #[test]
    #[allow(dead_code)]
    fn match_destructing_enums() {
        enum Color {
            Red,
            Blue,
            Green,
            RGB(u8, u8, u8),
            HSV(u8, u8, u8),
            HSL(u8, u8, u8),
            CMY(u8, u8, u8),
            CMYK(u8, u8, u8, u8),
        }

        let color = Color::RGB(122, 17, 40);
        match color {
            Color::Red => println!("The color is Red!"),
            Color::Blue => println!("The color is Blue!"),
            Color::Green => println!("The color is Green!"),
            Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}", r, g, b),
            Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}", h, s, v),
            Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}", h, s, l),
            Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}", c, m, y),
            Color::CMYK(c, m, y, k) => println!(
                "Cyan: {}, magenta: {}, yellow: {}, key (black): {}",
                c, m, y, k
            ),
        }
    }

    #[test]
    fn match_destructing_pointer() {
        let reference = &4;
        match reference {
            &val => println!("Got a value via destructuring: {}", val),
        }
        println!("reference: {:?}", reference);
        println!("*reference: {:?}", *reference);

        match *reference {
            val => println!("Got a value via dereferencing: {}", val),
        }

        let ref reference = 4;
        match reference {
            val => println!("Got a value via destructuring: {}", val),
        }

        let value = 5;
        let mut mut_value = 6;
        match value {
            ref r => println!("Got a reference to a value: {}", r),
        }

        match mut_value {
            ref mut m => {
                *m += 10;
                println!("We added 10. `mut_value`: {}", m);
            }
        }
    }

    #[test]
    fn match_struct() {
        struct Foo {
            x: (u32, u32),
            y: u32,
        }

        let foo = Foo { x: (1, 2), y: 3 };
        match foo {
            Foo { x: (1, b), y } => println!("First of x is `1`, b = {}, y = {}", b, y),
            Foo { y: 2, x: i } => println!("y is `2`, i = {:?}", i),
            Foo { y, .. } => println!("y = {}, we don't care about x", y),
        }
    }

    #[test]
    fn match_guards() {
        let pair = (2, -2);
        match pair {
            (x, y) if x == y => println!("These are twins"),
            (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
            (x, _) if x % 2 == 1 => println!("The first one is odd"),
            _ => println!("No correlation..."),
        }
    }

    #[test]
    fn match_binding() {
        let age = 21;
        match age {
            0 => println!("I'm not born yet I guess"),
            n @ 1..=12 => println!("I'm a child of age {}", n),
            n @ 13..=19 => println!("I'm a teen of age {}", n),
            n => println!("I'm an old person of age {}", n),
        }
    }

    #[test]
    fn if_let_optional() {
        let number = Some(7);
        if let Some(i) = number {
            println!("Matched: {:?}", i);
        }

        let letter: Option<i32> = None;
        if let Some(i) = letter {
            println!("Matched: {:?}", i);
        } else {
            println!("Didn't match");
        }

        let number = Some(7);
        match number {
            Some(i) => println!("Matched: {:?}", i),
            _ => (),
        }

        if let Some(i) = number {
            println!("Matched: {:?}", i);
        } else {
            println!("Didn't match");
        }
    }

    #[test]
    fn if_let_enum() {
        enum Foo {
            Bar,
            Baz,
            Qux(u32),
        }

        let a = Foo::Bar;
        let b = Foo::Baz;
        let c = Foo::Qux(100);

        if let Foo::Bar = a {
            println!("a is foobar");
        }

        // error[E0369]: binary operation `==` cannot be applied to type `flow_control::tests::if_let_enum::Foo`
        // if a == Foo::Bar {
        //     println!("a is foobar");
        // }

        if let Foo::Bar = b {
            println!("b is foobar");
        }

        if let Foo::Qux(value) = c {
            println!("c is {}", value);
        }
    }

    #[test]
    fn while_let() {
        let mut optional = Some(0);
        while let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
        }
    }
}
