use std::collections::HashMap;

use conf::App;
use error::MineError;
use lazy_static::lazy_static;
use poem::{listener::TcpListener, Server};
use result::MineResult;
use rust_embed::RustEmbed;
use tera::Tera;

pub mod conf;
pub mod consts;
pub mod enums;
pub mod error;
pub mod handler;
pub mod jwt;
pub mod middleware;
pub mod model;
pub mod result;
pub mod route;

lazy_static! {
    static ref APP: App = App::new().unwrap();
    static ref TEMPLATE: Tera = {
        let mut tera = Tera::new("resources/templates/**/*")
            .map_err(|e| MineError::E(e.to_string()))
            .unwrap();
        tera.autoescape_on(vec![".html"]);
        tera.register_function("ref", r#ref);
        // tera.full_reload().expect("tera full reload fail");
        tera
    };
}

#[derive(RustEmbed)]
#[folder = "resources/static"]
pub struct Statics;

#[derive(RustEmbed)]
#[folder = "resources/templates/"]
struct Templates;

/// 获取模板内容
pub fn template_data(name: &str) -> String {
    let data = match Templates::get(name) {
        Some(d) => d.data.into_owned(),
        None => "template data error".into(),
    };
    let data = std::str::from_utf8(data.as_ref()).unwrap_or("");
    data.to_string()
}

// pub static APP: OnceLock<App> = OnceLock::new();
// pub static TERA: OnceLock<Tera> = OnceLock::new();

// async fn init_app() -> MineResult<()> {
//     let app = App::new()?;
//     APP.get_or_init(|| app);
//     Ok(())
// }
//
// async fn init_template() -> MineResult<()> {
//     let mut tera =
//         Tera::new("resources/templates/**/*").map_err(|e| MineError::E(e.to_string()))?;
//     tera.autoescape_on(vec![".html"]);
//     TERA.get_or_init(|| tera);
//     Ok(())
// }

fn r#ref(args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let none = "/static/none".to_string();
    match args.get("path") {
        Some(v) => match tera::from_value::<String>(v.clone()) {
            Ok(v) => Ok(tera::Value::String(format!("/static/{v}"))),
            Err(_) => Ok(tera::Value::String(none)),
        },
        None => Ok(tera::Value::String(none)),
    }
}

pub async fn run() -> MineResult<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind(format!("{}:{}", APP.server.host, APP.server.port));
    let server = Server::new(listener);

    server
        .run(route::route())
        .await
        .map_err(|e| MineError::E(e.to_string()))
}
