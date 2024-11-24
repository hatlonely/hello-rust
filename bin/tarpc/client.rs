use ::tarpc::tokio_serde::formats::Json;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use tarpc::{client, context};

#[tarpc::service]
pub trait World {
    async fn hello(name: String) -> String;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
    let mut transport = tarpc::serde_transport::tcp::connect(addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    let client = WorldClient::new(client::Config::default(), transport.await?).spawn();
    let hello = client.hello(context::current(), "Stim".to_string()).await?;

    println!("{hello}");

    Ok(())
}
