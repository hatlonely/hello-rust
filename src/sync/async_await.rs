// https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html

#[cfg(test)]
mod tests {
    #[test]
    fn test_async_await() {
        async fn say_hello() {
            println!("Hello, world!");
        }

        async fn say_hello_delay() {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("Hello, world!");
        }

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let task1 = tokio::spawn(say_hello());
            let task2 = tokio::spawn(say_hello_delay());

            let _ = tokio::join!(task1, task2);
        });
    }
}
