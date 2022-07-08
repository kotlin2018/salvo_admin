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
    use crate::init_rbatis;

    #[test]
    async fn test(){
        let rb = init_rbatis().await;
        rb.fe
    }

}

