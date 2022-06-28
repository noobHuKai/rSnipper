use crate::api::{echo, get_tags, post_tags};
use poem::{get, post, Route};

pub fn init_router() -> Route {
    Route::new()
        .at("/echo", get(echo))
        .at("/tags", get(get_tags).post(post_tags))
}
