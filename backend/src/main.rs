use std::env;

use axum::{routing::get, Router, Server};
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions};
use anyhow::Result;

use serde::{Deserialize, Serialize};

mod route;
mod model;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(db_url.as_str())
        .await?;
    
    let app = Router::new()
        .route("/", get(route::test_handle))
        .route("/servers", get(route::servers))
        .route("/organizations", get(route::organizations))
        .with_state(pool);

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse()?)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}