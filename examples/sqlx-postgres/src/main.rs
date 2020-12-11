use async_trait::async_trait;
use tide::log;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Row;

use resourceable::{Resourceable, add_resource};

#[derive(Serialize, Deserialize)]
struct Spaceship {
    id: Option<i32>,
    num_thrusters: i32,
    name: String,
}

#[async_trait]
impl Resourceable<PgPool, i32> for Spaceship {
    async fn read_by_id(pool: &PgPool, id: i32) -> anyhow::Result<Spaceship> {
        let res = sqlx::query_as(
            r#"
                SELECT * FROM spaceship
                WHERE spaceship_id = $1
            "#,
        ).bind(id).fetch_one(pool).await?;

        Ok(res)
    }

    async fn read_paged(pool: &PgPool, size: u32, offset: u32) -> anyhow::Result<Vec<Spaceship>> {
        let res = sqlx::query_as(
            r#"
                SELECT * FROM spaceship
                LIMIT $1 OFFSET $2
            "#,
        ).bind(size).bind(offset).fetch_all(pool).await?;

        Ok(res)
    }

    async fn create(pool: &PgPool, spaceship: Spaceship) -> anyhow::Result<Spaceship> {
        let res = sqlx::query_as(
            r#"
                INSERT INTO spaceship (spaceship_num_thrusters, spaceship_name)
                VALUES ($1, $2)
                RETURNING *
            "#
        ).bind(spaceship.num_thrusters)
        .bind(spaceship.name)
        .fetch_one(pool)
        .await?;

        Ok(res)
    }

    async fn update(pool: &PgPool, id: i32, spaceship: Spaceship) -> anyhow::Result<Spaceship> {
        let res = sqlx::query_as(
            r#"
                UPDATE spaceship
                SET spaceship_num_thrusters = $2, spaceship_name = $3
                WHERE spaceship_id = $1
                RETURNING *
            "#,
        ).bind(id)
        .bind(spaceship.num_thrusters)
        .bind(spaceship.name)
        .fetch_one(pool).await?;

        Ok(res)
    }

    async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<Spaceship> {
        let res = sqlx::query_as(
            r#"
                DELETE FROM spaceship
                WHERE spaceship_id = $1
                RETURNING *
            "#,
        ).bind(id)
        .fetch_one(pool).await?;

        Ok(res)
    }
}

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Spaceship {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> sqlx::Result<Spaceship> {
        Ok(Spaceship {
            id: Some(row.try_get("spaceship_id")?),
            num_thrusters: row.try_get("spaceship_num_thrusters")?,
            name: row.try_get("spaceship_name")?,
        })
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();
    dotenv().ok();

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    let mut app = tide::with_state(pool);
    app.with(tide::log::LogMiddleware::new());
    add_resource!(app, "/spaceship", Spaceship);
    app.listen("127.0.0.1:8082").await?;
    Ok(())
}

