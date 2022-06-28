use poem::{
    handler,
    web::{Data, Json, Path},
};

use crate::{response::JsonResponse, service::Tag};
use sled::Db;

#[handler]
pub fn echo() -> Json<JsonResponse> {
    Json(JsonResponse::ok_with_string("biu! biu! biu!".to_string()))
}

#[handler]
pub fn get_tags(db: Data<&Db>) -> Json<JsonResponse> {
    match Tag::get_all(&db) {
        Ok(res) => Json(JsonResponse::ok_with_value(serde_json::json!(res))),
        Err(err) => Json(JsonResponse::fail(err.to_string())),
    }
}

#[handler]
pub fn post_tags(db: Data<&Db>, Path(tag): Path<String>) -> Json<JsonResponse> {
    match Tag::insert(&db, &tag) {
        Ok(_) => Json(JsonResponse::ok("ok".to_string())),
        Err(err) => Json(JsonResponse::fail(err.to_string())),
    }
}
