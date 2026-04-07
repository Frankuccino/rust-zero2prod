use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;


async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// pub async fn run() -> std::io::Result<()> {
pub fn run(address: &str) -> Result<Server, std::io::Error> {
    let server= HttpServer::new( || { 
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    // .bind(("127.0.0.1", 8000))?
    .bind(address)?
    .run();
    // .await // - No .await here!, we need to run our application as a background task.
    Ok(server)
}

