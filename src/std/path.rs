// https://doc.rust-lang.org/rust-by-example/std_misc/path.html

#[cfg(test)]
mod tests {
    #[test]
    fn test_path() {
        use std::path::Path;

        let path = Path::new(".");
        println!("path: {}", path.display());
        assert!(path.is_dir());
        assert!(path.exists());
        assert!(!path.is_absolute());
        assert!(path.canonicalize().is_ok());
        assert_eq!(path.display().to_string(), ".");

        let new_path = path.join("a").join("b");
        println!("new path: {}", new_path.display());
        assert_eq!(new_path.display().to_string(), "./a/b");

        let parent = new_path.parent().unwrap();
        println!("parent: {}", parent.display());
        assert_eq!(parent.display().to_string(), "./a");
    }
}
