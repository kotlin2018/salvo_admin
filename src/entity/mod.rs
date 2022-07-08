mod user;
mod role;
mod user_role;
mod user_post;
mod user_online;
mod user_dept;
mod update_log;
mod role_dept;
mod role_api;
mod post;
mod operate_log;
mod menu;
mod login_log;
mod job_log;
mod job;
mod dict_type;
mod dict_data;
mod dept;
mod api_db;

#[cfg(test)]
mod test {
    use std::ptr::null;
    use rbatis::crud::CRUD;
    use rbatis::{DateTimeNative, Uuid};
    use crate::entity::user::UserEntity;
    use crate::init_rbatis;

    #[tokio::test]
    async fn test(){
        let rb = init_rbatis().await;
        let result = rb.fetch_by_column::<UserEntity,_>("id","b70f1684-e616-4cae-9b24-a65dabead1dd").await;
        println!("{:?}",result)
    }

    #[tokio::test]
    async fn test_insert(){
        let rb = init_rbatis().await;
        let user = UserEntity{
            id: Some(Uuid::new()),
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
}

