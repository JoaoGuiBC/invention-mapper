use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, put, web};
use sea_orm::DatabaseConnection;

use crate::handlers::{HandlerReturn, invention_handlers::update_invention};
use crate::models::invention::UpdateInvention;

use super::INVENTION_SWAGGER_TAG;

/// Update a invention data
#[utoipa::path(
    path = "/api/invention",
    tag = INVENTION_SWAGGER_TAG,
    operation_id = "update_invention",
    request_body(content = UpdateInvention, content_type = "multipart/form-data"),
    responses(
        (status = 200, body = String, description = "INVENÇÃO EDITADA COM SUCESSO"),
        (status = 400, body = String, description = "ERRO AO LER ID, CONFIRA SE OS DADOS ESTÃO CORRETOS"),
        (status = 404, body = String, description = "INVENÇÃO NÃO ENCONTRADA NO BANCO DE DADOS"),
        (status = 500, body = String, description = "ERRO AO SALVAR IMAGEM. TENTE NOVAMENTE MAIS TARDE."),
        (status = 500, body = String, description = "ERRO AO ATUALIZAR NO BANDO DE DADOS. TENTE NOVAMENTE MAIS TARDE."),
    )
)]
#[put("")]
async fn route(
    db_conn: web::Data<DatabaseConnection>,
    MultipartForm(invention): MultipartForm<UpdateInvention>,
) -> impl Responder {
    let new_invention = update_invention::execute(&db_conn, invention).await;

    match new_invention {
        HandlerReturn::Success(content) => HttpResponse::build(content.0).json(content.1),
        HandlerReturn::Failure(content) => HttpResponse::build(content.0).json(content.1),
    }
}
