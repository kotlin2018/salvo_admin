#[macro_use]
extern crate rbatis;
use salvo::prelude::*;
use crate::controller::init_router;
use crate::entity::{ApplicationConfig, init_rbatis};

mod entity;
mod controller;
mod service;
mod dto;

#[tokio::main]
async fn main() {
    let rb = init_rbatis().await;
    let server_url = ApplicationConfig::default().server_url;
    tracing_subscriber::fmt::init();
    tracing::info!("Listening on {:?}",&server_url);//这里传 server_url 的引用
    Server::new(TcpListener::
    bind(&server_url)).serve(init_router()).await;
}
