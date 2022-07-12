use salvo::prelude::*;

#[fn_handler]
pub async fn cors(req: &mut Request,res: &mut Response){
    let origin = match req.header::<String>("Origin") {
        Some(origin) => {
            let header_mut = req.headers_mut();
            header_mut.insert("Access-Control-Allow-Origin", origin.parse().unwrap());
            header_mut.insert("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PUT, DELETE, UPDATE".parse().unwrap());
            header_mut.insert("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept, Authorization".parse().unwrap());
            header_mut.insert("Access-Control-Expose-Headers", "Content-Length, Access-Control-Allow-Origin, Access-Control-Allow-Headers, Cache-Control, Content-Language, Content-Type".parse().unwrap());
            header_mut.insert("Access-Control-Allow-Credentials", "true".parse().unwrap());
            header_mut.insert("Access-Control-Max-Age", "86400".parse().unwrap());
            if req.method().as_str() == "OPTIONS" {
                res.set_status_code("200".parse().unwrap());
            }
        },
        None => {
            res.render(Text::Plain("设置跨域失败"))
        }
    };
}



