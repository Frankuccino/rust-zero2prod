use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use std::net::TcpListener;


async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

// We were returning `impl Responder` at the very beginning.
// We are now spelling out the type explicityly given that we have become more familiar with actix-web
// There is no performance difference! Just a stylistic choice
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}



// Let's start simple: we always return a 200 OK
// async fn subscribe() -> HttpResponse {
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
    // format!("Welcome {}", form.username)
}
// pub async fn run() -> std::io::Result<()> {
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server= HttpServer::new( || { 
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    // .bind(("127.0.0.1", 8000))?
    .listen(listener)?
    .run();
    // .await // - No .await here!, we need to run our application as a background task.
    Ok(server)
}

