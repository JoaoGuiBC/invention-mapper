use actix_web::http::StatusCode;

pub mod invention_handlers;

pub enum HandlerReturn<T> {
    Success((StatusCode, T)),
    Failure((StatusCode, String)),
}
