use crate::init_rbatis;

/// 秘密加密
pub fn encrypt_password(password: &str,salt: &str) -> String{
    use std::fmt::Write;
    let s = password.to_owned() + salt;
    let digest = md5::compute(s).to_vec();
    let mut result = String::new();
    for a in digest.iter(){
        write!(result,"{:02x}",a).unwrap();
    }
    result
}

/// 用户登陆
pub async fn login(){
    // 根据用户名获取用户信息
    let rb = init_rbatis().await;
    let w = rb.new_wrapper().eq("user_name", &login_req.user_name);
    let user = rb.fetch_by_wrapper::<UserEntity>(w).await.unwrap();
    //let user = rb.fetch_by_column::<UserEntity,_>("user_name", &login_req.user_name).await.unwrap();
    let user1 = user.clone();
    let user2 = user.clone();
    if &user.user_status.unwrap() == "0" {
        msg = "用户已经被禁用".to_string();
        status = "0".to_string();
        res.code = Some(400);
        res.msg = Some(msg);
        res.data = Some(user1);
        Json(res)
    }else {
        res.code = Some(200);
        res.msg = Some("success".to_string());
        res.data = Some(user2);
        Json(res)
    }
}


