use salvo::prelude::*;
use crate::dao::{init_rbatis,request_data::SearchReq,request_data::PageParams};

// #[fn_handler]
// pub async fn get_sort_list(page_params: PageParams,search: Search) ->Json<ListData<UserResp>>{
//   let rb = init_rbatis().await;
//
//   Json(page_params)
// }

#[fn_handler]
pub async fn get_sort_list(page_params: PageParams,search: SearchReq){
  // let rb = init_rbatis().await;
  //   rb.

}

#[fn_handler]
pub async fn get_by_id(req: &mut Request){
}

#[fn_handler]
pub async fn get_profile(req: &mut Request){
}

// #[fn_handler]
// pub async fn get_profile(req: &mut Request) -> Json<T>{
//   Json(None)
// }





