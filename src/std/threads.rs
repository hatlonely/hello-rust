// https://doc.rust-lang.org/rust-by-example/std_misc/threads.html

#[cfg(test)]
mod tests {
    #[test]
    fn test_threads() {
        use std::thread;

        let mut ts = vec![];

        for i in 0..10 {
            ts.push(thread::spawn(move || {
                println!("this is thread number {}", i)
            }));
        }

        for t in ts {
            let _ = t.join();
        }
    }
}
