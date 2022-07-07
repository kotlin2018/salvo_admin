use serde::{Serialize,Deserialize};
use salvo::macros::Extractible;
use tokio::fs::OpenOptions;

/// 分页参数
#[derive(Debug,Serialize,Deserialize,Extractible)]
//#[extract(default_source(from = "body", format = "json"))]
pub struct PageParams {
    pub page_num: Option<usize>,
    pub page_size: Option<usize>,
    pub sort: Option<String>,
}

#[derive(Debug,Serialize,Deserialize,Extractible)]
pub struct Search {

}