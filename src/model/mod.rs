use async_static::async_static;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

pub mod user;

async_static! {
    static ref DB: Pool<Postgres> = pool().await;
}

async fn pool() -> Pool<Postgres> {
    sqlx::postgres::PgPool::connect("postgres://postgres:postgres@127.0.0.1/mine")
        .await
        .unwrap()
}

/// 时间类型
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TimeType {
    /// 当前
    #[default]
    ToDay,
    /// 最近一周
    LastWeek,
    /// 最近一个月
    LastMonth,
    /// 最近一季度
    LastQuarter,
    /// 最近一年
    LastYear,
    /// 自定义
    Custom,
}

impl TimeType {
    /// 根据时间类型计算开始和结束时间
    pub fn calc(&self) -> (u64, u64) {
        match self {
            TimeType::ToDay => (0, 0),
            TimeType::LastWeek => todo!(),
            TimeType::LastMonth => todo!(),
            TimeType::LastQuarter => todo!(),
            TimeType::LastYear => todo!(),
            TimeType::Custom => todo!(),
        }
    }
}

/// 分页返回封装
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PageRes<T> {
    page: i64,
    page_size: i64,
    total: i64,
    list: Vec<T>,
    first: bool,
    last: bool,
    has_next: bool,
    has_pre: bool,
    total_pages: i64,
}

impl<T> std::default::Default for PageRes<T> {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 15,
            total: 0,
            list: vec![],
            first: true,
            last: false,
            has_next: false,
            has_pre: false,
            total_pages: 0,
        }
    }
}

impl<T> PageRes<T>
where
    T: Serialize + Clone,
{
    pub fn new(total: i64, mut page: i64, page_size: i64, list: &[T]) -> Self {
        if page <= 0 {
            page = 1;
        }
        let total_pages = (total as f64 / page_size as f64).ceil() as i64;
        Self {
            page,
            page_size,
            total,
            list: list.iter().cloned().collect::<Vec<_>>(),
            first: page == 1,
            last: page == total_pages,
            has_next: page < total_pages,
            has_pre: page > 1,
            total_pages,
        }
    }
}
