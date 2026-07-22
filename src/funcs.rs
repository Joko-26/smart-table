use axum::{http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use sqlx::{Executor, Pool, Sqlite};

pub async fn db() -> Pool<Sqlite> {
    let opt = sqlx::sqlite::SqliteConnectOptions::new()
        .filename("data/app.db")
        .create_if_missing(true);
    let pool = sqlx::sqlite::SqlitePool::connect_with(opt).await.unwrap();

    pool.execute(
        "
      CREATE TABLE if not exists users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT
      )
    ",
    )
    .await
    .unwrap();

    pool
}

pub fn format_date(date: DateTime<Utc>) -> String {
    let new_date = date.format("%d-%b-%Y %H:%M:%S %P %z");
    return new_date.to_string();
}

pub async fn ping() -> impl IntoResponse {
    let json_response = serde_json::to_string_pretty("pong").unwrap();
    (StatusCode::OK, json_response).into_response()
}
