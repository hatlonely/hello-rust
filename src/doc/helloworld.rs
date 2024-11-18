#[cfg(test)]
mod tests {
    #[test]
    fn helloworld() {
        println!("Hello, world!");
    }

    #[test]
    fn print() {
        println!("{} days", 31);
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
        println!(
            "{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        );

        println!(
            "{} of {:b} people know binary, the other half doesn't",
            1, 2
        );

        println!("Base 10: {}", 69420);
        println!("Base 10: {:}", 69420);
        println!("Base 2: {:b}", 69420);
        println!("Base 8: {:o}", 69420);
        println!("Base 16: {:x}", 69420);

        println!("right justify: {n:0>5}", n = 1);
        println!("left justify: {n:0<5}", n = 1);
        println!("center justify: {n:0^5}", n = 1);
        println!("justify with variable: {n:0>width$}", n = 1, width = 5);
    }
}
