use poem::{
    handler,
    web::{Data, Json, Path},
};

use crate::response::JsonResponse;
use crate::service::TagService;
use sqlx::{Pool, Sqlite};

#[handler]
pub fn echo() -> Json<JsonResponse> {
    Json(JsonResponse::ok_with_string("biu! biu! biu!".to_string()))
}

#[handler]
pub async fn get_tags(db: Data<&Pool<Sqlite>>) -> Json<JsonResponse> {
    match TagService::get_all(&db).await {
        Ok(res) => Json(JsonResponse::ok_with_value(serde_json::json!(res))),
        Err(err) => Json(JsonResponse::fail(err.to_string())),
    }
}

#[handler]
pub async fn post_tags(db: Data<&Pool<Sqlite>>, Path(tag): Path<String>) -> Json<JsonResponse> {
    match TagService::insert(&db, tag).await {
        Ok(_) => Json(JsonResponse::ok("ok".to_string())),
        Err(err) => Json(JsonResponse::fail(err.to_string())),
    }
}
