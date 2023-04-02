//! src/routes/subscriptions.rs

use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(
    _form: web::Form<FormData>,
    // Retrieving a connection from the application state!
    _connection: web::Data<PgConnection>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
