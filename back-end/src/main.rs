use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Server};

mod api;
mod response;
mod router;
mod service;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // open database
    let db: sled::Db = sled::open("rSnipperDB")?;

    // db.insert("key", "v1").unwrap();

    //  start server
    Server::new(TcpListener::bind("0.0.0.0:43214"))
        .run(router::init_router().with(AddData::new(db)))
        .await?;

    Ok(())
}
