use actix_web::{get, HttpResponse, Responder};

#[get("/health-check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
