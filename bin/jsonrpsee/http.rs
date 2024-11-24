use jsonrpsee::server::{RpcModule, Server};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct EchoRequest {
    message: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct EchoResponse {
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = Server::builder().build("127.0.0.1:8081").await?;

    let mut module = RpcModule::new(());

    // curl -X POST -H "Content-Type: application/json" \
    //   -d '{"jsonrpc":"2.0","method":"say_hello","params":[],"id":1}' \
    //   http://127.0.0.1:8081
    module.register_method("say_hello", |_, _, _| {
        println!("say_hello method called!");
        "Hello there!!"
    })?;

    // curl -X POST -H "Content-Type: application/json" \
    //   -d '{"jsonrpc":"2.0","method":"echo","params":{"message":"hatlonely"},"id":1}' \
    //   http://127.0.0.1:8081
    module.register_method("echo", |params, _, _| {
        let request: EchoRequest = serde_json::from_str(params.as_str().unwrap()).unwrap();
        let response = EchoResponse {
            message: request.message.clone(),
        };
        println!("echo method called! {:?}", &request.message);
        serde_json::to_string(&response)
            .map_err(|e| e.to_string())
            .unwrap()
    })?;

    let handler = server.start(module);

    handler.stopped().await;

    Ok(())
}
