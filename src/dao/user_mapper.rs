use rbatis::{Page, PageRequest};
use rbatis::executor::RbatisExecutor;
use rbatis::rbatis::Rbatis;
use rbson::Bson;
use crate::dao::request_data::SearchReq;
use crate::dao::response_data::{UserAndDeptResp};
use crate::entity::user::UserEntity;

#[html_sql("./src/mapper/user_mapper.html")]
pub async fn get_sort_list(rb: &mut RbatisExecutor<'_,'_>,page_req: &PageRequest,sql_arg: &SearchReq) -> Page<UserAndDeptResp>{impled!()}

#[html_sql("./src/mapper/user_mapper.html")]
pub async fn select_one(mut rb: RbatisExecutor<'_,'_>,name: &str) -> UserEntity {impled!()}

#[py_sql("select * from sys_user where user_name =#{name}")]
pub async fn fetch_one(mut rb: Rbatis,name: &str) -> UserEntity{impled!()}

#[cfg(test)]
mod test{
    use rbatis::crud::CRUD;
    use rbatis::{DateTimeNative, PageRequest, Uuid};
    use crate::dao::request_data::SearchReq;
    use crate::dao::user_mapper::{fetch_one, get_sort_list, select_one};
    use crate::entity::user::UserEntity;
    use crate::init_rbatis;

    #[tokio::test]
    async fn test_insert(){
        let rb = init_rbatis().await;
        let user = UserEntity{
            //id: Some(Uuid::new()),
            id: Some(Uuid::new().to_string()),
            user_name: Some("admin".to_string()),
            user_nickname: Some("马磊".to_string()),
            user_password: Some("123456".to_string()),
            user_salt: Some("4#)MGcEj2p".to_string()),
            user_status: Some("1".to_string()),
            user_email: Some("123456789@136.com".to_string()),
            sex: Some("男".to_string()),
            avatar: Some("/upload/2022-03/10-0105B3BSNSQ6LVEEG4GG3UU1KI.jpg".to_string()),
            role_id: None,
            dept_id: None,
            remark: None,
            is_admin: None,
            phone_num: None,
            last_login_ip: None,
            last_login_time: None,
            created_at: Some(DateTimeNative::now()),
            updated_at: None,
            deleted_at: None
        };
        let r = rb.save(&user,&[]).await;
        if r.is_err() {
            println!("{}",r.err().unwrap().to_string())
        }
    }

    #[tokio::test]
    async fn test(){
        fast_log::init(fast_log::config::Config::new().console()).expect("TODO: panic message");
        let rb = init_rbatis().await;
        let params = SearchReq{
            user_id: Some("00TV87DDOBJPU75J4TGUOC3NNG".to_string()),
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

    #[tokio::test]
    async fn test02(){
        fast_log::init(fast_log::config::Config::new().console()).expect("TODO: panic message");
        let rb = init_rbatis().await;
        let user_name = "admin";
        let w = rb.new_wrapper().eq("user_name", &user_name);
        let user = rb.fetch_list_by_wrapper::<UserEntity>(w).await.unwrap();
        println!("{:?}",user);
    }

    #[tokio::test]
    async fn test03(){
        fast_log::init(fast_log::config::Config::new().console()).expect("TODO: panic message");
        let rb = init_rbatis().await;
        let user = fetch_one(rb,"admin").await;
        println!("{:?}",user);
    }

    #[tokio::test]
    async fn test04(){
        fast_log::init(fast_log::config::Config::new().console()).expect("TODO: panic message");
        let rb = init_rbatis().await;
        let user = select_one(rb.as_executor(),"admin").await;
        println!("{:?}",user);
    }
}

