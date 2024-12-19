use actix_web::{get, HttpResponse, Responder};

#[get("/route")]
pub async fn route() -> impl Responder {
    HttpResponse::Ok().finish()
}
