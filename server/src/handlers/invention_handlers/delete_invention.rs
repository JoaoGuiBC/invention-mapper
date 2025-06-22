use actix_web::http::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use entity::invention::Entity as invention_entity;

use crate::handlers::HandlerReturn;

pub async fn execute(db_conn: &DatabaseConnection, invention_id: Uuid) -> HandlerReturn<String> {
    let invention = invention_entity::find_by_id(invention_id)
        .one(db_conn)
        .await;

    match invention {
        Ok(invention) => {
            if invention == None {
                return HandlerReturn::Failure((
                    StatusCode::NOT_FOUND,
                    "INVENÇÃO NÃO ENCONTRADA NO BANCO DE DADOS".to_string(),
                ));
            }

            let deleted_invention = invention_entity::delete_by_id(invention_id)
                .exec(db_conn)
                .await;

            match deleted_invention {
                Ok(_) => {
                    return HandlerReturn::Success((
                        StatusCode::OK,
                        "INVENÇÃO DELETADA COM SUCESSO".to_string(),
                    ));
                }
                Err(error) => {
                    println!("{error}");

                    return HandlerReturn::Failure((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "ERRO AO DELETAR DO BANDO DE DADOS. TENTE NOVAMENTE MAIS TARDE."
                            .to_string(),
                    ));
                }
            }
        }
        Err(error) => {
            println!("{error}");

            return HandlerReturn::Failure((
                StatusCode::INTERNAL_SERVER_ERROR,
                "ERRO AO DELETAR DO BANDO DE DADOS. TENTE NOVAMENTE MAIS TARDE.".to_string(),
            ));
        }
    }
}
