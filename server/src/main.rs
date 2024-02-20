mod db;
mod consts;

use ntex::web::{self, App, HttpServer, Responder};
use crate::consts::DB_URL;

fn create_user() -> impl Responder{

}
fn create_room() -> impl Responder{

}
fn send_message() -> impl Responder{

}
fn block_user() -> impl Responder{

}

#[ntex::main]
async fn main() {
    use migration::{Migrator, MigratorTrait};

    let connection = sea_orm::Database::connect(DB_URL).await?;
    Migrator::up(&connection, None).await?;

    // let db = Database(pool);

    HttpServer::new(move || {
        App::new()
            .route("/create_user", web::post().to(create_user))
            .route("/send_message", web::post().to(send_message))
            .route("/create_room", web::post().to(create_room))
            .route("/block_user", web::post().to(block_user))

        // ... WebSocket upgrade route
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
