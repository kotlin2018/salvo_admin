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
    // let addr = format!("{}:{}",&CONFIG.application.host.unwrap(),&CONFIG.application.port.unwrap());
    // Server::new(TcpListener::bind(&addr)).serve(router()).await;
    // info!("Local: http://{}",addr);
}
