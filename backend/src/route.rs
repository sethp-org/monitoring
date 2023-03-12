use axum::extract::State;
use sqlx::MySqlPool;
use anyhow::Result;

use crate::error::AppError;
use crate::model::Server;


pub async fn test_handle() -> Result<&'static str, ()> {
    Ok("hello")
}

pub async fn servers(
    State(pool): State<MySqlPool>,
) -> Result<String, AppError> {
    let stream = sqlx::query_as::<_, Server>("SELECT * FROM servers")
    .fetch_all(&pool).await?;

    Ok(serde_json::to_string(&stream)?)
}

pub async fn organizations(
    State(pool): State<MySqlPool>,
) -> Result<String, AppError> {
    let stream = sqlx::query_as::<_, Server>("SELECT * FROM organizations")
    .fetch_all(&pool).await?;

    Ok(serde_json::to_string(&stream)?)
}