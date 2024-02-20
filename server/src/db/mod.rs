use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn setup_connection() -> DatabaseConnection{
    let mut opt = ConnectOptions::new("protocol://rakanor:busywoods-1799@host/database");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(60))
        .acquire_timeout(Duration::from_secs(60))
        .idle_timeout(Duration::from_secs(60))
        .max_lifetime(Duration::from_secs(660))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema");

    Database::connect(opt).await?
}

pub async fn close_connection(conn: DatabaseConnection){
    conn.close().await?
}

pub async fn verify_connection(conn: DatabaseConnection){
    assert!(conn.ping().await.is_ok());
    conn.clone().close().await?;
    assert!(matches!(conn.ping().await?, Err(DbErr::ConnectionAcquire)));
}