use poem::{async_trait, Endpoint, IntoResponse, Middleware, Request, Response, Result};

use crate::{jwt::TokenData, APP};

/// TODO 接口权限校验中间件
pub async fn auth_middleware<E: Endpoint>(next: E, req: Request) -> Result<E::Output> {
    log::info!("接口权限校验中间件");
    next.call(req).await
}

/// token校验中间件
pub async fn token_middleware<E: Endpoint>(next: E, mut req: Request) -> Result<E::Output> {
    log::info!("token中间件");
    let uri = req.uri().path();
    // 白名单不校验token
    let wl = APP.white_list.to_vec();
    if !wl.is_empty() && wl.iter().all(|u| !uri.starts_with(u)) {
        if let Some(value) = req
            .headers()
            .get("X-Token")
            .and_then(|value| value.to_str().ok())
        {
            match TokenData::verify(value) {
                Ok(token) => {
                    req.extensions_mut().insert(token);
                }
                Err(e) => {
                    log::error!("验证token有效失败: uri = {uri}, token = {value}, {e}");
                    // FIXME
                    return Err(poem::Error::from_string(
                        e.to_string(),
                        poem::http::StatusCode::BAD_REQUEST,
                    ));
                }
            }
        }
    }
    next.call(req).await
}

/// 日志中间件
pub struct LogMilldeware;

impl<E: Endpoint> Middleware<E> for LogMilldeware {
    type Output = LogImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        LogImpl(ep)
    }
}

pub struct LogImpl<E>(E);

#[async_trait]
impl<E: Endpoint> Endpoint for LogImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        println!("请求: {}", req.uri().path());
        let res = self.0.call(req).await;

        match res {
            Ok(resp) => {
                let resp = resp.into_response();
                println!("响应: {}", resp.status());
                Ok(resp)
            }
            Err(err) => {
                println!("错误: {}", err);
                Err(err)
            }
        }
    }
}
