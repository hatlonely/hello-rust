#[cfg(test)]
mod tests {
    #[test]

    fn test_futures() {
        async fn download_async() -> Result<String, &'static str> {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            Ok("Hello, world!".to_string())
        }

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let result = download_async().await;
            assert_eq!(result, Ok("Hello, world!".to_string()));
        });
    }
}
