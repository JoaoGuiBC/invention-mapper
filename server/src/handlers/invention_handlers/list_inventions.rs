use actix_web::http::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};

use entity::invention::Entity as invention_entity;

use crate::handlers::HandlerReturn;
use crate::models::invention::Invention;

pub async fn execute(db_conn: &DatabaseConnection) -> HandlerReturn<Vec<Invention>> {
    let inventions = invention_entity::find().all(db_conn).await;

    match inventions {
        Ok(inventions) => {
            let inventions = inventions
                .into_iter()
                .map(|invention_model| Invention {
                    id: invention_model.id,
                    name: invention_model.name,
                    text: invention_model.text,
                    external_link: invention_model.external_link,
                    lat: invention_model.lat,
                    lon: invention_model.lon,
                })
                .collect();

            return HandlerReturn::Success((StatusCode::OK, inventions));
        }
        Err(error) => {
            println!("{error}");

            return HandlerReturn::Failure((
                StatusCode::INTERNAL_SERVER_ERROR,
                "ERRO BUSCAR DO BANDO DE DADOS. TENTE NOVAMENTE MAIS TARDE.".to_string(),
            ));
        }
    }
}
