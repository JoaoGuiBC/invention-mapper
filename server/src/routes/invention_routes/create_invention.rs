use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, post, web};
use sea_orm::DatabaseConnection;

use crate::handlers::{HandlerReturn, invention_handlers::create_invention};
use crate::models::invention::{CreateInvention, Invention};

use super::INVENTION_SWAGGER_TAG;

/// Add a new invention
#[utoipa::path(
    path = "/api/invention",
    tag = INVENTION_SWAGGER_TAG,
    operation_id = "create_invention_from_manual_insert",
    request_body(content = CreateInvention, content_type = "multipart/form-data"),
    responses(
        (status = 201, body = Invention, description = "Invention object"),
        (status = 500, body = String, description = "ERRO AO SALVAR NO BANDO DE DADOS. TENTE NOVAMENTE MAIS TARDE."),
    )
)]
#[post("")]
async fn route(
    db_conn: web::Data<DatabaseConnection>,
    MultipartForm(invention): MultipartForm<CreateInvention>,
) -> impl Responder {
    let new_invention = create_invention::execute(&db_conn, invention).await;

    match new_invention {
        HandlerReturn::Success(content) => HttpResponse::build(content.0).json(content.1),
        HandlerReturn::Failure(content) => HttpResponse::build(content.0).json(content.1),
    }
}
