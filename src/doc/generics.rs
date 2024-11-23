// https://doc.rust-lang.org/rust-by-example/generics/gen_trait.html

#[cfg(test)]
mod tests {
    #[test]
    fn functions() {
        fn add<T>(a: T, b: T) -> T
        where
            T: std::ops::Add<Output = T>,
        {
            a + b
        }

        println!("add(1, 2): {}", add(1, 2));
        println!("add(1.1, 2.2): {}", add(1.1, 2.2));
    }
}
