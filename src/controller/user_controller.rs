use salvo::prelude::*;
use crate::controller::ListData;
use crate::controller::response_data::UserResp;

#[fn_handler]
pub async fn get_sort_list() ->Json<ListData<UserResp>>{

  Json()
}

#[fn_handler]
pub async fn get_by_id(req: &mut Request) -> Json<T>{

}

#[fn_handler]
pub async fn get_profile(req: &mut Request) -> Json<T>{

}





