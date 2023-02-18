use rbatis::crud;

pub struct CasbinRule{
    pub p_type: Option<String>,
    pub v0: Option<String>,
    pub v1: Option<String>,
    pub v2: Option<String>,
    pub v3: Option<String>,
    pub v4: Option<String>,
    pub v5: Option<String>,
}
crud!(CasbinRule{});