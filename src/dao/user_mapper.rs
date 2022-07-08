use rbatis::{Page, PageRequest};
use rbatis::executor::RbatisExecutor;
use rbson::Bson;

#[html_sql("./src/mapper/user_mapper.html")]
pub async fn get_sort_list(rb: &mut RbatisExecutor<'_,'_>,page_req: &PageRequest) -> Page<String>{impled!()}
