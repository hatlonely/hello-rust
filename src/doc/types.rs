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
}
