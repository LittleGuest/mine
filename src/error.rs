//! 全局异常

#[derive(Debug, thiserror::Error)]
pub enum MineError {
    #[error("{0}")]
    E(String),
    #[error("序列化错误")]
    SerializeError,
    #[error("验证码错误")]
    CaptchaError,
    #[error("验证码失效")]
    CaptchaExpireError,
    #[error("账号或密码错误")]
    UsernameOrPasswordError,
    #[error("用户不存在")]
    UserNotFound,
    #[error("{0}")]
    JwtError(&'static str),
    #[error("服务器异常 : {0}")]
    ServerError(&'static str),
    #[error("SQL错误")]
    SqlError,
    #[error("未知错误")]
    Unknown,

    // #[error(transparent)]
    // PoemError(#[from] poem::Error),
    #[error("参数校验错误: {0}")]
    ValidationError(&'static str),
    // #[error(transparent)]
    // AnyhowError(#[from] anyhow::Error),
}
