use log::info;
use salvo::prelude::*;
use crate::config::CONFIG;
use crate::handler::router;

pub mod handler;
pub mod logic;
pub mod entity;
pub mod config;
pub mod middleware;

#[tokio::main]
async fn main() {
    let host = &CONFIG.application.host;
    let port = &CONFIG.application.port;
    let addr = format!("{}:{}",host,port);
    println!("Local: http://{}",addr);
    Server::new(TcpListener::bind(&addr)).serve(router()).await;
}
