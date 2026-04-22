use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgConnection;

// Bring the re-exported routes into scope
use crate::routes::{health_check, subscribe};

// Startup will host the run function
pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let connection = web::Data::new(connection);

    let server= HttpServer::new( move || { 
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    // .bind(("127.0.0.1", 8000))?
    .listen(listener)?
    .run();
    // .await // - No .await here!, we need to run our application as a background task.
    Ok(server)
}