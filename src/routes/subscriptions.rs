use actix_web::{HttpResponse, web};
use sqlx::PgConnection;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

// Let's start simple: we always return a 200 OK
pub async fn subscribe(_form: web::Form<FormData>, _connection: web::Data<PgConnection>) -> HttpResponse {
    HttpResponse::Ok().finish()
}