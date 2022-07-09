use rbatis::{Page, PageRequest};
use rbatis::executor::RbatisExecutor;
use rbson::Bson;
use crate::dao::request_data::SearchReq;
use crate::dao::response_data::{UserAndDeptResp};

#[html_sql("./src/mapper/user_mapper.html")]
pub async fn get_sort_list(rb: &mut RbatisExecutor<'_,'_>,page_req: &PageRequest,sql_arg: &SearchReq) -> Page<UserAndDeptResp>{impled!()}

#[cfg(test)]
mod test{
    use rbatis::PageRequest;
    use crate::dao::request_data::SearchReq;
    use crate::dao::user_mapper::get_sort_list;
    use crate::init_rbatis;

    #[tokio::test]
    async fn test(){
        let rb = init_rbatis().await;
        let params = SearchReq{
            user_id: Some("b70f1684-e616-4cae-9b24-a65dabead1dd".to_string()),
            role_id: None,
            user_ids: None,
            user_name: None,
            phone_num: None,
            user_nickname: None,
            user_status: None,
            dept_id: None,
            begin_time: None,
            end_time: None
        };
        let res = get_sort_list(&mut rb.as_executor(),
                                &PageRequest::new(1,10),&params).await;
        println!("{:?}",res);
    }
}

