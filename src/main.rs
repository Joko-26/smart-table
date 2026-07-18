use std::env;
use dotenv::dotenv;
use axum::{routing::get, Router};
use sqlx::{Pool, Sqlite};

mod funcs;


fn app(_pool: Pool<Sqlite>) -> Router {
    Router::new()
    .route("/ping", get(funcs::ping))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("running");
    let pool = funcs::db().await;
    let app = app(pool);
    let listener = tokio::net::TcpListener::bind(env::var("URL").unwrap()).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("running on: {}", env::var("URL").unwrap())
}
