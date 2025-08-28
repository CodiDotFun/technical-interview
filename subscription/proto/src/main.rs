use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tonic::transport::server::TcpIncoming;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let db_url = "sqlite://interview.db";
    let db = SqlitePool::connect(db_url)
        .await
        .expect("Failed to connect to database");
