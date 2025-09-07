use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use crate::models::{Item, Meta, Link};
use chrono::Utc;

//--------------------------------------------------------------------------------
// Payloads
//--------------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct CreateItem {
    pub meta_id: i64,
    pub item_seq: i32,
    pub item_sq2: i32,
    pub item_name: String,
    pub item_desc: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateItem {
    pub item_seq: i32,
    pub item_sq2: i32,
    pub item_name: String,
    pub item_desc: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateLink {
    pub meta_id: i64,
    pub link_l_id: i64,
    pub link_r_id: i64,
    pub link_seq: i32,
    pub link_sq2: i32,
    pub link_desc: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateLink {
    pub link_l_id: i64,
    pub link_r_id: i64,
    pub link_seq: i32,
    pub link_sq2: i32,
    pub link_desc: Option<String>,
}


//--------------------------------------------------------------------------------
// Item Handlers
//--------------------------------------------------------------------------------

#[axum::debug_handler]
pub async fn create_item(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateItem>,
) -> Result<Json<Item>, (StatusCode, String)> {
    let now = Utc::now();
    let item = sqlx::query_as::<_, Item>(
        "INSERT INTO items (meta_id, item_seq, item_sq2, item_name, item_desc, updated_at, update_count) VALUES (?, ?, ?, ?, ?, ?, 1) RETURNING *",
    )
    .bind(payload.meta_id)
    .bind(payload.item_seq)
    .bind(payload.item_sq2)
    .bind(payload.item_name)
    .bind(payload.item_desc)
    .bind(now.to_rfc3339())
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(item))
}

pub async fn list_items(State(pool): State<SqlitePool>) -> Result<Json<Vec<Item>>, (StatusCode, String)> {
    let items = sqlx::query_as("SELECT * FROM items").fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(items))
}

pub async fn get_item(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Item>, (StatusCode, String)> {
    let item = sqlx::query_as("SELECT * FROM items WHERE id = ?").bind(id).fetch_one(&pool).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Item with id {} not found", id)),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    })?;
    Ok(Json(item))
}

#[axum::debug_handler]
pub async fn update_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateItem>,
) -> Result<Json<Item>, (StatusCode, String)> {
    let now = Utc::now();
    let item = sqlx::query_as(
        "UPDATE items SET item_seq = ?, item_sq2 = ?, item_name = ?, item_desc = ?, updated_at = ?, update_count = update_count + 1 WHERE id = ? RETURNING *",
    )
    .bind(payload.item_seq)
    .bind(payload.item_sq2)
    .bind(payload.item_name)
    .bind(payload.item_desc)
    .bind(now.to_rfc3339())
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(item))
}

pub async fn delete_item(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM items WHERE id = ?").bind(id).execute(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}


//--------------------------------------------------------------------------------
// Link Handlers
//--------------------------------------------------------------------------------

#[axum::debug_handler]
pub async fn create_link(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateLink>,
) -> Result<Json<Link>, (StatusCode, String)> {
    let now = Utc::now();
    let link = sqlx::query_as(
        "INSERT INTO links (meta_id, link_l_id, link_r_id, link_seq, link_sq2, link_desc, updated_at, update_count) VALUES (?, ?, ?, ?, ?, ?, ?, 1) RETURNING *",
    )
    .bind(payload.meta_id)
    .bind(payload.link_l_id)
    .bind(payload.link_r_id)
    .bind(payload.link_seq)
    .bind(payload.link_sq2)
    .bind(payload.link_desc)
    .bind(now.to_rfc3339())
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(link))
}

pub async fn list_links(State(pool): State<SqlitePool>) -> Result<Json<Vec<Link>>, (StatusCode, String)> {
    let links = sqlx::query_as("SELECT * FROM links").fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(links))
}

pub async fn get_link(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Link>, (StatusCode, String)> {
    let link = sqlx::query_as("SELECT * FROM links WHERE link_id = ?").bind(id).fetch_one(&pool).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Link with id {} not found", id)),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    })?;
    Ok(Json(link))
}

#[axum::debug_handler]
pub async fn update_link(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateLink>,
) -> Result<Json<Link>, (StatusCode, String)> {
    let now = Utc::now();
    let link = sqlx::query_as(
        "UPDATE links SET link_l_id = ?, link_r_id = ?, link_seq = ?, link_sq2 = ?, link_desc = ?, updated_at = ?, update_count = update_count + 1 WHERE link_id = ? RETURNING *",
    )
    .bind(payload.link_l_id)
    .bind(payload.link_r_id)
    .bind(payload.link_seq)
    .bind(payload.link_sq2)
    .bind(payload.link_desc)
    .bind(now.to_rfc3339())
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(link))
}

pub async fn delete_link(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM links WHERE link_id = ?").bind(id).execute(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}


//--------------------------------------------------------------------------------
// Meta Handlers
//--------------------------------------------------------------------------------

pub async fn list_metas(State(pool): State<SqlitePool>) -> Result<Json<Vec<Meta>>, (StatusCode, String)> {
    let metas = sqlx::query_as("SELECT * FROM metas").fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(metas))
}

pub async fn get_meta(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Meta>, (StatusCode, String)> {
    let meta = sqlx::query_as("SELECT * FROM metas WHERE meta_id = ?").bind(id).fetch_one(&pool).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Meta with id {} not found", id)),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    })?;
    Ok(Json(meta))
}
