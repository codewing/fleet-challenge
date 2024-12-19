use actix_web::{get, HttpResponse, Responder};

#[get("/validate-layout")]
pub async fn validate_layout() -> impl Responder {
    HttpResponse::Ok().finish()
}
