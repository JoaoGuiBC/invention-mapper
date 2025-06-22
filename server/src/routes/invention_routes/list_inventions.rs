use actix_web::{HttpResponse, Responder, get, web};
use sea_orm::DatabaseConnection;

use crate::handlers::{HandlerReturn, invention_handlers::list_inventions};
use crate::models::invention::Invention;

use super::INVENTION_SWAGGER_TAG;

/// List all inventions
#[utoipa::path(
    path = "/api/invention",
    tag = INVENTION_SWAGGER_TAG,
    operation_id = "list_inventions",
    responses(
        (status = 200, body = Vec<Invention>, description = "List of inventions"),
        (status = 500, body = String, description = "ERRO BUSCAR DO BANDO DE DADOS. TENTE NOVAMENTE MAIS TARDE."),
    )
)]
#[get("")]
pub async fn route(db_conn: web::Data<DatabaseConnection>) -> impl Responder {
    let inventions = list_inventions::execute(&db_conn).await;

    match inventions {
        HandlerReturn::Success(content) => HttpResponse::build(content.0).json(content.1),
        HandlerReturn::Failure(content) => HttpResponse::build(content.0).json(content.1),
    }
}
