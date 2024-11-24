// https://doc.rust-lang.org/rust-by-example/std/box.html

#[cfg(test)]
mod tests {
    #[test]
    fn test_box() {
        let heap_x = Box::new(5);
        assert_eq!(5, *heap_x);

        let stack_x = 5;
        assert_eq!(5, stack_x);
    }
}
