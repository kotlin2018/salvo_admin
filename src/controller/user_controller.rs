use salvo::prelude::*;
use crate::controller::ListData;
use crate::controller::request_data::{PageParams, Search};
use crate::controller::response_data::UserResp;

// #[fn_handler]
// pub async fn get_sort_list(page_params: PageParams,search: Search) ->Json<ListData<UserResp>>{
//
//   Json(page_params)
// }

#[fn_handler]
pub async fn get_sort_list(page_params: PageParams,search: Search){
  println!("{:?},{:?}",page_params,search);
}

// #[fn_handler]
// pub async fn get_by_id(req: &mut Request) -> Json<T>{
//   Json(None)
// }

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





