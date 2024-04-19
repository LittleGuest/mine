use poem::{
    error::InternalServerError,
    handler,
    web::{Html, Json},
    Body, IntoResponse,
};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::{result::MineResult, TEMPLATE};

#[handler]
pub async fn html_login() -> MineResult<Html<String>, poem::Error> {
    let mut context = Context::new();
    TEMPLATE
        .render("login.html", &context)
        .map_err(InternalServerError)
        .map(Html)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginBody {
    account: String,
    password: String,
    captcha: String,
}

#[handler]
pub async fn login(
    Json(LoginBody {
        account,
        password,
        captcha,
    }): Json<LoginBody>,
) -> impl IntoResponse {
    let login_body = LoginBody {
        account,
        password,
        captcha,
    };
    Json(login_body)
}
