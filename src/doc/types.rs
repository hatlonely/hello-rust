#[cfg(test)]
mod tests {
    #[test]
    fn casting() {
        let decimal = 65.4321_f32;

        let integer = decimal as u8;
        let character = integer as char;
        println!("Casting: {} -> {} -> {}", decimal, integer, character);

        unsafe {
            let decimal = 300.0_f32;
            let integer = decimal.to_int_unchecked::<u8>();  // 溢出
            print!("Casting: {} -> {}", decimal, integer);
        }
    }

    #[test]
    fn literals() {
        let x = 1u8;
        let y = 2u32;
        let z = 3f32;

        let i = 1;
        let f = 1.0;

        println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
        println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
        println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
        println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
        println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    }

    #[test]
    fn inference() {
        let elem = 5u8;

        let mut vec = Vec::new();
        vec.push(elem);  // 根据 `elem` 推断 `vec` 的类型
        println!("{:?}", vec);
    }

    #[test]
    fn aliasing() {
        type NanoSecond = u64;
        type Inch = u64;

        // 使用别名
        let nanoseconds: NanoSecond = 5 as u64;
        let inches: Inch = 2 as u64;

        // 类型别名不提供额外的类型安全，因为别名不提供新的类型
        println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
    }
}
