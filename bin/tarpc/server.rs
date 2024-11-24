use futures::prelude::*;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::{
    context,
    server::{self, incoming::Incoming, Channel},
    tokio_serde::formats::Json,
};

#[derive(Clone)]
struct HelloServer();

#[tarpc::service]
pub trait World {
    async fn hello(name: String) -> String;
}

impl World for HelloServer {
    async fn hello(self, _: context::Context, name: String) -> String {
        format!("Hello, {name}!")
    }
}

async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50051);
    let mut listener = tarpc::serde_transport::tcp::listen(addr, Json::default).await?;

    listener.config_mut().max_frame_length(usize::MAX);
    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = HelloServer();
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
