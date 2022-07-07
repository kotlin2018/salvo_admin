use salvo::Router;
pub mod user;

// 初始化路由
pub fn init_router() ->Router{
   let router = Router::new();

    router
}
