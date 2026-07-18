use std::env;
use dotenv::dotenv;
use axum::{routing::get, Router};
use sqlx::{Pool, Sqlite};

mod funcs;


fn app(_pool: Pool<Sqlite>) -> Router {
    Router::new()
    .route("/ping", get(funcs::ping))
    /* 
    .route("/tasks/{user-id}")
    .route("/tests/{user-id}")
    .route("/user/{id}")
    .route("/timetable/{user-id}")
    .route("/sujects/{user-id}")
    .route("config/{user-id}")
    .route("/mix/tasks/{user-id}")
    .route("/mix/tests/{user-id}")
    .route("/mix/user/{id}")
    .route("/mix/timetable/{user-id}")
    .route("/mix/sujects/{user-id}")
    .route("/mix/config/{user-id}")
    */
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
