use rbatis::{Page, PageRequest};
use rbatis::rbatis::Rbatis;

#[html_sql("../mapper/user_mapper.html")]
pub async fn get_sort_list(rb: &mut Rbatis,page_req: &PageRequest) -> Page<String>{impled!()}