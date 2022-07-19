#[macro_use]
extern crate rbatis;
use salvo::prelude::*;
use crate::controller::init_router;
use crate::entity::{ApplicationConfig, init_rbatis};
#[allow(dead_code,unused)]
mod entity;
#[allow(dead_code,unused)]
mod controller;
#[allow(dead_code,unused)]
mod service;
#[allow(dead_code,unused)]
mod dto;
mod helper;

#[tokio::main]
async fn main() ->Result<(),Box<dyn std::error::Error>>{
    let server_url = ApplicationConfig::default().server_url;
    tracing_subscriber::fmt::init();
    tracing::info!("Listening on {:?}",&server_url);//这里传 server_url 的引用
    Server::new(TcpListener::
    bind(&server_url)).serve(init_router()).await;
    Ok(())
}
