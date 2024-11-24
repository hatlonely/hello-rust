// https://doc.rust-lang.org/rust-by-example/std/str.html

#[cfg(test)]
mod tests {
    #[test]
    fn test_string_create() {
        let s1 = String::from("hello");
        assert_eq!(s1, "hello");

        let s2 = "world".to_string();
        assert_eq!(s2, "world");

        let s3 = String::with_capacity(10);
        assert_eq!(s3.capacity(), 10);
    }

    #[test]
    fn test_string_attributes() {
        let s = String::from("hello");
        assert_eq!(s.len(), 5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.contains("hello"), true);
        assert_eq!(s.contains("world"), false);
        assert_eq!(s.starts_with("he"), true);
        assert_eq!(s.ends_with("lo"), true);
    }

    #[test]
    fn test_string_modify() {
        let mut s = String::from("hello");

        s.push_str(" world");
        assert_eq!(s, "hello world");

        s.pop();
        assert_eq!(s, "hello worl");

        s.insert(5, 'd');
        assert_eq!(s, "hellod worl");

        s.remove(5);
        assert_eq!(s, "hello worl");

        let n = s.replace("wo", "wooo");
        assert_eq!(n, "hello wooorl");
        assert_eq!(s, "hello worl");
    }

    #[test]
    fn test_string_escape() {
        let s = "hello\nworld";
        assert_eq!(s, "hello\nworld");

        let s = r"hello\nworld";
        assert_eq!(s, "hello\\nworld");
    }
}
