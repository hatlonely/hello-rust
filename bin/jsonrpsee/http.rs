use jsonrpsee::server::{RpcModule, Server};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Message {
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = Server::builder().build("127.0.0.1:8081").await?;

    let mut module = RpcModule::new(());
    module.register_method("say_hello", |_, _, _| {
        println!("say_hello method called!");
        "Hello there!!"
    })?;

    let handler = server.start(module);

    handler.stopped().await;

    Ok(())
}
