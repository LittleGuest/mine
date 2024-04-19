use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use super::DB;
use crate::{error::MineError, result::MineResult};

/// 用户
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    FromRow,
    Validate,
)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct User {
    ///
    pub id: Option<u32>,
    /// 账号
    #[validate(length(max = 50))]
    pub account: Option<String>,
    /// 密码
    #[validate(length(max = 62))]
    pub password: Option<String>,
    /// 昵称
    #[validate(length(max = 100))]
    pub nickname: Option<String>,
    /// 头像
    #[validate(length(max = 150))]
    pub avatar: Option<String>,
    /// 签名
    #[validate(length(max = 200))]
    pub sign: Option<String>,
    /// 1-启用，0-禁用
    pub status: Option<bool>,
    /// 添加时间
    pub create_at: Option<u64>,
    /// 修改时间
    pub update_at: Option<u64>,
    /// 删除时间
    pub delete_at: Option<u64>,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::json!(self))
    }
}

impl User {
    fn table_name() -> String {
        "user".to_string()
    }

    fn columns() -> String {
        "id,account,password,nickname,avatar,sign,status,create_at,update_at,delete_at".to_string()
    }

    pub async fn fetch_by_id(id: u64) -> MineResult<Self> {
        let sql = format!(
            "select {} from {} where id = ?",
            Self::columns(),
            Self::table_name()
        );

        sqlx::query_as::<_, Self>(&sql)
            .bind(id)
            .fetch_one(DB.await)
            .await
            .map_err(|e| {
                log::error!("{e}");
                MineError::SqlError
            })
    }

    pub async fn fetch_all(req: &UserReq) -> MineResult<Vec<Self>> {
        let mut sql = format!("select {} from {}", Self::columns(), Self::table_name());

        let mut where_sql = " WHERE 1=1 ".to_string();

        if let Some(id) = &req.id {
            where_sql.push_str(&format!(" and {} = {} ", "id", id));
        }

        if let Some(account) = &req.account {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "account", account));
        }

        if let Some(password) = &req.password {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "password", password));
        }

        if let Some(nickname) = &req.nickname {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "nickname", nickname));
        }

        if let Some(avatar) = &req.avatar {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "avatar", avatar));
        }

        if let Some(sign) = &req.sign {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "sign", sign));
        }

        if let Some(status) = &req.status {
            where_sql.push_str(&format!(" and {} = {} ", "status", status));
        }

        if let Some(create_at) = &req.create_at {
            where_sql.push_str(&format!(" and {} = {} ", "create_at", create_at));
        }

        if let Some(update_at) = &req.update_at {
            where_sql.push_str(&format!(" and {} = {} ", "update_at", update_at));
        }

        if let Some(delete_at) = &req.delete_at {
            where_sql.push_str(&format!(" and {} = {} ", "delete_at", delete_at));
        }

        sql.push_str(&where_sql);

        sqlx::query_as::<_, Self>(&sql)
            .fetch_all(DB.await)
            .await
            .map_err(|e| {
                log::error!("{e}");
                MineError::SqlError
            })
    }

    pub async fn insert(&mut self) -> MineResult<Self> {
        let sql = format!(
            "INSERT INTO {} ({}) VALUES({})",
            Self::table_name(),
            Self::columns(),
            "?,?,?,?,?,?,?,?,?,?,".trim_end_matches(',')
        );
        let id = sqlx::query(&sql)
            .bind(&self.id)
            .bind(&self.id)
            .bind(&self.account)
            .bind(&self.password)
            .bind(&self.nickname)
            .bind(&self.avatar)
            .bind(&self.sign)
            .bind(&self.status)
            .bind(&self.create_at)
            .bind(&self.update_at)
            .bind(&self.delete_at)
            .execute(DB.await)
            .await
            .map_err(|e| {
                log::error!("{e}");
                MineError::SqlError
            })?
            .rows_affected();
        Self::fetch_by_id(id).await
    }

    pub async fn update(&mut self) -> MineResult<bool> {
        let sql = format!(
            "UPDATE {} set account = ?, set {} where id = ?",
            Self::table_name(),
            "id = ?,account = ?,password = ?,nickname = ?,avatar = ?,sign = ?,status = ?,create_at = ?,update_at = ?,delete_at = ?,".trim_end_matches(',')
        );
        sqlx::query(&sql)
            .bind(&self.id)
            .bind(&self.account)
            .bind(&self.password)
            .bind(&self.nickname)
            .bind(&self.avatar)
            .bind(&self.sign)
            .bind(&self.status)
            .bind(&self.create_at)
            .bind(&self.update_at)
            .bind(&self.delete_at)
            .bind(&self.id)
            .execute(DB.await)
            .await
            .map_err(|e| {
                log::error!("{e}");
                MineError::SqlError
            })
            .map(|r| r.rows_affected() > 0)
    }

    pub async fn delete(&self) -> MineResult<bool> {
        let sql = format!("DELETE FROM {} WHERE id = ?", Self::table_name());
        sqlx::query(&sql)
            .bind(self.id)
            .execute(DB.await)
            .await
            .map_err(|e| {
                log::error!("{e}");
                MineError::SqlError
            })
            .map(|r| r.rows_affected() > 0)
    }

    async fn count(where_sql: &str) -> MineResult<(i64,)> {
        let count_sql = format!(
            "SELECT count(*) FROM {} WHERE {}",
            Self::table_name(),
            where_sql
        );

        sqlx::query_as::<_, (i64,)>(&count_sql)
            .fetch_one(DB.await)
            .await
            .map_err(|e| {
                log::error!("{e}");
                MineError::SqlError
            })
    }

    pub async fn page(req: &UserReq) -> MineResult<super::PageRes<Self>> {
        let mut where_sql = " 1 = 1 ".to_string();

        if let Some(id) = &req.id {
            where_sql.push_str(&format!(" and {} = {} ", "id", id));
        }

        if let Some(account) = &req.account {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "account", account));
        }

        if let Some(password) = &req.password {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "password", password));
        }

        if let Some(nickname) = &req.nickname {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "nickname", nickname));
        }

        if let Some(avatar) = &req.avatar {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "avatar", avatar));
        }

        if let Some(sign) = &req.sign {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "sign", sign));
        }

        if let Some(status) = &req.status {
            where_sql.push_str(&format!(" and {} = {} ", "status", status));
        }

        if let Some(create_at) = &req.create_at {
            where_sql.push_str(&format!(" and {} = {} ", "create_at", create_at));
        }

        if let Some(update_at) = &req.update_at {
            where_sql.push_str(&format!(" and {} = {} ", "update_at", update_at));
        }

        if let Some(delete_at) = &req.delete_at {
            where_sql.push_str(&format!(" and {} = {} ", "delete_at", delete_at));
        }

        let (count,) = Self::count(&where_sql).await?;

        let mut page = req.page.unwrap_or(0) - 1;
        let page_size = req.page_size.unwrap_or(15);
        if page < 0 {
            page = 0;
        }
        where_sql.push_str(&format!(" LIMIT {}, {} ", page * page_size, page_size));

        let res = match count > 0 {
            true => {
                let mut sql = format!(
                    "SELECT {} FROM {} WHERE ",
                    Self::columns(),
                    Self::table_name()
                );

                sql.push_str(&where_sql);
                sqlx::query_as::<_, Self>(&sql)
                    .fetch_all(DB.await)
                    .await
                    .map_err(|e| {
                        log::error!("{e}");
                        MineError::SqlError
                    })?
            }
            false => Vec::new(),
        };
        Ok(super::PageRes::new(count, page, page_size, &res))
    }
}

impl User {
    pub async fn fetch_by_account(account: &str) -> MineResult<Self> {
        let sql = format!(
            "select {} from {} where account = ?",
            Self::columns(),
            Self::table_name()
        );
        sqlx::query_as::<_, Self>(&sql)
            .bind(account)
            .fetch_one(DB.await)
            .await
            .map_err(|e| {
                log::error!("{e}");
                MineError::SqlError
            })
    }
}

/// 用户
#[derive(
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Validate,
)]
pub struct UserReq {
    pub time_type: Option<super::TimeType>,
    /// 开始时间
    pub start_at: Option<u64>,
    /// 结束时间
    pub end_at: Option<u64>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,

    ///
    pub id: Option<u32>,
    /// 账号
    pub account: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 昵称
    pub nickname: Option<String>,
    /// 头像
    pub avatar: Option<String>,
    /// 签名
    pub sign: Option<String>,
    /// 1-启用，0-禁用
    pub status: Option<bool>,
    /// 添加时间
    pub create_at: Option<u64>,
    /// 修改时间
    pub update_at: Option<u64>,
    /// 删除时间
    pub delete_at: Option<u64>,
}
