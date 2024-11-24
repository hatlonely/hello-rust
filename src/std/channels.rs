// https://doc.rust-lang.org/rust-by-example/std_misc/channels.html

#[cfg(test)]
mod tests {
    #[test]
    fn test_channels() {
        use std::sync::mpsc;
        use std::sync::mpsc::{Receiver, Sender};
        use std::thread;

        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let mut ts = vec![];

        for i in 0..10 {
            let tx = tx.clone();
            ts.push(thread::spawn(move || {
                tx.send(i).unwrap();
                println!("send {}", i);
            }));
        }

        for _ in 0..10 {
            println!("received {}", rx.recv().unwrap());
        }

        for t in ts {
            let _ = t.join();
        }
    }
}
