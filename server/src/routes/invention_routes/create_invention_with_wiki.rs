use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, post, web};
use sea_orm::DatabaseConnection;

use crate::handlers::HandlerReturn;
use crate::handlers::invention_handlers::create_invention_with_wiki;
use crate::models::invention::{CreateInventionWithWiki, Invention};

use super::INVENTION_SWAGGER_TAG;

/// Add a new invention using a wikipedia link
#[utoipa::path(
    path = "/api/invention/wiki",
    tag = INVENTION_SWAGGER_TAG,
    operation_id = "create_invention_from_wikipedia_link",
    request_body(content = CreateInventionWithWiki, content_type = "multipart/form-data"),
    responses(
        (status = 201, body = Invention, description = "Invention object"),
        (status = 500, body = String, description = "ERRO AO SALVAR NO BANDO DE DADOS. TENTE NOVAMENTE MAIS TARDE."),
    )
)]
#[post("/wiki")]
async fn route(
    db_conn: web::Data<DatabaseConnection>,
    MultipartForm(invention): MultipartForm<CreateInventionWithWiki>,
) -> impl Responder {
    let new_invention = create_invention_with_wiki::execute(&db_conn, invention).await;

    match new_invention {
        HandlerReturn::Success(content) => HttpResponse::build(content.0).json(content.1),
        HandlerReturn::Failure(content) => HttpResponse::build(content.0).json(content.1),
    }
}
