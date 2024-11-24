// https://doc.rust-lang.org/rust-by-example/std/hash/hashset.html

#[cfg(test)]
mod tests {
    #[test]
    fn test_rc() {
        use std::rc::Rc;

        let five = Rc::new(5);

        let _ = Rc::clone(&five);
        let _ = Rc::clone(&five);

        assert_eq!(5, *five);
    }

    #[test]
    fn test_arc() {
        use std::sync::Arc;
        use std::thread;

        let five = Arc::new(5);

        let _ = Arc::clone(&five);
        let _ = Arc::clone(&five);

        let _ = thread::spawn(move || {
            let _ = five;
        });

        assert_eq!(5, *five);
    }
}
