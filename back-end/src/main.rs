use anyhow::Result;
use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Server};
use sqlx::sqlite::SqlitePoolOptions;

mod api;
mod response;
mod router;
mod service;
mod common;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    // open database
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://rSnipper.db")
        .await?;

    //  start server
    Server::new(TcpListener::bind("0.0.0.0:43214"))
        .run(router::init_router().with(AddData::new(db)))
        .await?;

    Ok(())
}
