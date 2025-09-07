use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Meta {
    pub meta_id: i64,
    pub meta_l_id: i64,
    pub meta_r_id: i64,
    pub meta_type: i32,
    pub meta_name: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub update_count: i32,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub meta_id: i64,
    pub item_seq: i32,
    pub item_sq2: i32,
    pub item_name: String,
    pub item_desc: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub update_count: i32,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Link {
    pub link_id: i64,
    pub meta_id: i64,
    pub link_l_id: i64,
    pub link_r_id: i64,
    pub link_seq: i32,
    pub link_sq2: i32,
    pub link_desc: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub update_count: i32,
}
