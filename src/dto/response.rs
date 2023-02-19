use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize,Serialize};
use crate::error::Error;

#[derive(Debug,Clone,Deserialize)]
pub struct DataPermission{
    pub data_scope: Option<String>,
    pub user_id: Option<i32>,
    pub dept_id: Option<i32>,
    pub role_id: Option<i32>,
}

/// JWT authentication Token structure
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct JWTToken {
    //账号id
    pub id: i32,
    //账号
    pub account: String,
    // //过期时间
    pub exp: usize,
}

impl JWTToken {
    pub fn create_token(&self, secret: &str) -> String {
        let token = encode(&Header::default(),self,&EncodingKey::from_secret(secret.as_ref())).unwrap();
        token
    }

    pub fn verify(secret: &str, token: &str) -> Result<JWTToken, Error>{
        let validation = Validation::default();
        match decode::<JWTToken>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation){
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => Err(Error::from("无效的 token")),
                _=> Err(Error::from("其他错误")),
            }
        }
    }
}

