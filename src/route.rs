use poem::{endpoint::EmbeddedFilesEndpoint, get, post, Endpoint, EndpointExt, Route};

use crate::{handler, middleware, Statics};

pub fn route() -> impl Endpoint {
    Route::new()
        .at("/", get(handler::index::index))
        .nest("/static", EmbeddedFilesEndpoint::<Statics>::new())
        .nest("/blog", web())
        .nest("/console", console())
        .around(middleware::auth_middleware)
        .around(middleware::token_middleware)
        .with(middleware::LogMilldeware)
}

/// web
fn web() -> impl Endpoint {
    Route::new().at("/", get(handler::index::index))
}

/// console
fn console() -> impl Endpoint {
    Route::new()
        .at("/", get(handler::auth::html_login))
        .at("/login", get(handler::auth::html_login))
        .nest("/api", console_api())
}

fn console_api() -> impl Endpoint {
    Route::new()
        .at("/login", post(handler::auth::login))
        .at("/user/list", post(handler::user::user_list))
        .at("/user/page", post(handler::user::page))
}
