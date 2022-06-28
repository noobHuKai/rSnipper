use poem::{
    handler,
    web::{Data, Json},
};

use crate::{response::JsonResponse, service::Tag};
use sled::Db;

#[handler]
pub fn echo() -> Json<JsonResponse> {
    Json(JsonResponse::ok_with_string("biu! biu! biu!".to_string()))
}

#[handler]
pub fn get_tags(db: Data<&Db>) -> Json<JsonResponse> {
    println!(
        "{:?}",
        std::str::from_utf8(&db.get("$tags").unwrap().unwrap().to_vec())
    );
    Json(JsonResponse::ok_with_value(serde_json::json!([
        "java", "python", "go"
    ])))
}

#[handler]
pub fn post_tags(db: Data<&Db>) -> Json<JsonResponse> {
    Tag::insert(&db, "aaa").unwrap();

    Json(JsonResponse::ok_with_string("biu! biu! biu!".to_string()))
}
