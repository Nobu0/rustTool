mod models;
mod handlers;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::env;
use std::net::SocketAddr;
use crate::handlers::{
    // Items
    create_item,
    get_item,
    list_items,
    update_item,
    delete_item,
    // Links
    create_link,
    get_link,
    list_links,
    update_link,
    delete_link,
    // Metas
    get_meta,
    list_metas,
};

async fn create_schema(db_pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS metas (
            meta_id INTEGER PRIMARY KEY AUTOINCREMENT,
            meta_l_id INTEGER NOT NULL DEFAULT 0,
            meta_r_id INTEGER NOT NULL DEFAULT 0,
            meta_type INTEGER NOT NULL,
            meta_name TEXT,
            updated_at TEXT NOT NULL,
            update_count INTEGER NOT NULL DEFAULT 0
        )
        "#,
    )
    .execute(db_pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            meta_id INTEGER NOT NULL,
            item_seq INTEGER NOT NULL,
            item_sq2 INTEGER NOT NULL,
            item_name TEXT NOT NULL,
            item_desc TEXT,
            updated_at TEXT NOT NULL,
            update_count INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (meta_id) REFERENCES metas (meta_id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(db_pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS links (
            link_id INTEGER PRIMARY KEY AUTOINCREMENT,
            meta_id INTEGER NOT NULL,
            link_l_id INTEGER NOT NULL,
            link_r_id INTEGER NOT NULL,
            link_seq INTEGER NOT NULL,
            link_sq2 INTEGER NOT NULL,
            link_desc TEXT,
            updated_at TEXT NOT NULL,
            update_count INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (meta_id) REFERENCES metas (meta_id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(db_pool)
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect(".env file not found");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    if let Err(e) = create_schema(&pool).await {
        eprintln!("Failed to create schema: {}", e);
        return Err(Box::new(e));
    }
    println!("Database schema created successfully.");

    // API Router
    let app = Router::new()
        // Item routes
        .route("/items", get(list_items).post(create_item))
        .route("/items/:id", get(get_item).put(update_item).delete(delete_item))
        // Link routes
        .route("/links", get(list_links).post(create_link))
        .route("/links/:id", get(get_link).put(update_link).delete(delete_link))
        // Meta routes
        .route("/metas", get(list_metas))
        .route("/metas/:id", get(get_meta))
        .with_state(pool);

    // サーバーを起動
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
