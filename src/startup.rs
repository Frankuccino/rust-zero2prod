use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use std::net::TcpListener;

// Bring the re-exported routes into scope
use crate::routes::{health_check, subscribe};

// Startup will host the run function
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server= HttpServer::new( || { 
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    // .bind(("127.0.0.1", 8000))?
    .listen(listener)?
    .run();
    // .await // - No .await here!, we need to run our application as a background task.
    Ok(server)
}