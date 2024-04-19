use poem::{handler, web::Json, IntoResponse};

use crate::model::{self, user::UserReq};

#[handler]
pub async fn page(Json(req): Json<UserReq>) -> impl IntoResponse {
    let users = model::user::User::page(&req).await.unwrap();
    Json(users)
}

#[handler]
pub async fn user_list(Json(req): Json<UserReq>) -> impl IntoResponse {
    let users = model::user::User::fetch_all(&req).await.unwrap();
    Json(users)
}
