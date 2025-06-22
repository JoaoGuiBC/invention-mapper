use actix_web::{HttpResponse, Responder, delete, web};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::handlers::{HandlerReturn, invention_handlers::delete_invention};

use super::INVENTION_SWAGGER_TAG;

/// Add a new invention
#[utoipa::path(
    path = "/api/invention/{invention_id}",
    tag = INVENTION_SWAGGER_TAG,
    operation_id = "delete_invention",
    params(("invention_id" = String, Path, description = "Invention id. UUID format")),
    responses(
        (status = 200, body = String, description = "INVENÇÃO DELETADA COM SUCESSO"),
        (status = 400, body = String, description = "ID INVÁLIDO"),
        (status = 404, body = String, description = "INVENÇÃO NÃO ENCONTRADA NO BANDO DE DADOS"),
        (status = 500, body = String, description = "ERRO AO DELETAR DO BANDO DE DADOS. TENTE NOVAMENTE MAIS TARDE."),
    )
)]
#[delete("/{invention_id}")]
async fn route(
    db_conn: web::Data<DatabaseConnection>,
    invention_id: web::Path<String>,
) -> impl Responder {
    let invention_id = invention_id.into_inner();
    let invention_id = Uuid::parse_str(&invention_id);

    match invention_id {
        Ok(invention_id) => {
            let new_invention = delete_invention::execute(&db_conn, invention_id).await;

            match new_invention {
                HandlerReturn::Success(content) => HttpResponse::build(content.0).json(content.1),
                HandlerReturn::Failure(content) => HttpResponse::build(content.0).json(content.1),
            }
        }
        Err(_) => HttpResponse::BadRequest().json("ID INVÁLIDO"),
    }
}
