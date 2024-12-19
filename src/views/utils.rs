use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder};
use serde::Serialize;

pub fn to_response<T>(object: T, status_code: StatusCode) -> HttpResponse
where
    T: Sized + Serialize,
{
    HttpResponseBuilder::new(status_code).json(&object)
}
