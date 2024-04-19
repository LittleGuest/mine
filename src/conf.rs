//! 应用配置

use serde::{Deserialize, Serialize};

use crate::{error::MineError, result::MineResult};

/// 服务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    // 服务名称
    pub name: Option<String>,
    // 服务地址
    pub host: String,
    // 服务端口
    pub port: u16,
}

/// 逻辑删除字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logic {
    // 列名
    pub column: String,
    // 正常
    pub normal: u8,
    // 已删除
    pub deleted: u8,
}

/// 应用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub server: Server,
    // 数据库地址
    pub database_url: String,
    pub logic: Option<Logic>,
    // redis地址
    pub redis_url: String,
    // 日志目录
    pub log_dir: String,
    // JWT 密钥
    pub jwt_secret: String,
    // 白名单
    pub white_list: Vec<String>,
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl App {
    pub fn new() -> MineResult<Self> {
        let config = include_str!("../resources/conf.toml");
        toml_edit::de::from_str::<App>(config).map_err(|e| MineError::E(e.to_string()))
    }
}
