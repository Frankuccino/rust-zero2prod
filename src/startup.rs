use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use std::net::TcpListener;
use sqlx::PgPool;

// Bring the re-exported routes into scope
use crate::routes::{health_check, subscribe};

// Startup will host the run function
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);

    let server= HttpServer::new( move || { 
        App::new()
            // Middlewares are added using the `wrap` method on `App`
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    // .bind(("127.0.0.1", 8000))?
    .listen(listener)?
    .run();
    // .await // - No .await here!, we need to run our application as a background task.
    Ok(server)
}