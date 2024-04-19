use poem::{handler, web::Html, IntoResponse};
use tera::Context;

use crate::TEMPLATE;

#[handler]
pub async fn index() -> impl IntoResponse {
    let mut context = Context::new();
    context.insert("message", "hello");
    let body = TEMPLATE.render("error.html", &context).unwrap();
    Html(body)
}
