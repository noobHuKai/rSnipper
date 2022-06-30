use crate::api::{delete_tag, echo, get_tag, post_tag};
use poem::{get, post, Route};

pub fn init_router() -> Route {
    Route::new()
        .at("/echo", get(echo))
        .at("/tags", get(get_tag))
        .at("/tags/:tag", post(post_tag).delete(delete_tag))
}
