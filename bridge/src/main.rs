use std::net::{IpAddr, SocketAddr};

use anyhow::{Context, Result};
use axum::serve;
use mt5_bridge::Mt5Client;

mod server;

#[derive(Debug)]
struct Config {
    socket: SocketAddr,
    secret: String,
}

async fn init_config() -> Result<Config> {
    let _ = dotenvy::dotenv();
    let host: IpAddr = std::env::var("HOST")
        //
        .context("env:HOST is undefined")?
        .parse()
        .context("env:HOST is not ip address")?;
    let port: u16 = std::env::var("PORT")
        //
        .context("env:PORT is undefined")?
        .parse()
        .context("env:PORT is not port number")?;
    let socket = SocketAddr::from((host, port));
    let secret: String = std::env::var("MT5_SECRET")
        //
        .context("MT5_SECRET is undefined")?
        .to_string();
    //
    Ok(Config { socket, secret })
}

async fn init_bridge(config: &Config) -> Result<Mt5Client> {
    let client = mt5_bridge::Mt5Client::connect_with_secret(config.secret.as_str())
        .context("can not initialize mt5 client")?;
    Ok(client)
}

async fn init_server(config: &Config, client: &Mt5Client) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(config.socket)
        .await
        .context("can not bind tcp listener")?;
    serve(listener, server::create_router(&client))
        .await
        .context("can not initialize http server")?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = init_config()
        .await
        .context("initialize environment is failed")?;
    let client = init_bridge(&config)
        .await
        .context("initialize mt5 client is failed")?;
    let _ = init_server(&config, &client)
        .await
        .context("initialize http server is failed")?;
    Ok(())
}
