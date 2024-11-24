// https://doc.rust-lang.org/rust-by-example/std_misc/process.html

#[cfg(test)]
mod tests {
    #[test]
    fn test_processes() {
        use std::process::Command;

        let output = Command::new("ls")
            .arg("-l")
            .arg("-a")
            .output()
            .expect("failed to execute process");

        assert!(output.status.success());
        assert!(output.stdout.len() > 0);
        assert_eq!(output.stderr.len(), 0);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    }

    #[test]
    fn test_wait() {
        use std::process::Command;

        let mut child = Command::new("sleep").arg("1").spawn().unwrap();

        let ecode = child.wait().unwrap();

        assert!(ecode.success());
    }
}
