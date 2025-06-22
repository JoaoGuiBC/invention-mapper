use actix_multipart::form::tempfile::TempFile;
use actix_web::http::StatusCode;
use cloudinary::upload::{OptionalParameters, Source, Upload, UploadResult};
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::collections::BTreeSet;
use uuid::Uuid;

use entity::invention as invention_entity;

use crate::env::ENV;
use crate::handlers::HandlerReturn;
use crate::models::invention::UpdateInvention;

pub async fn upload_image(image_file: TempFile, invention_id: &String) -> Result<(), String> {
    let file_path = image_file.file.path().to_path_buf();

    let options = BTreeSet::from([
        OptionalParameters::Folder(ENV.cloudinary_images_folder.clone()),
        OptionalParameters::PublicId(invention_id.to_owned()),
    ]);
    let upload = Upload::new(
        ENV.cloudinary_api_key.clone(),
        ENV.cloudinary_cloud_name.clone(),
        ENV.cloudinary_api_secret.clone(),
    );
    let result = upload.image(Source::Path(file_path), &options).await;

    match result {
        Ok(result) => match result {
            UploadResult::Response(_) => Ok(()),
            UploadResult::ResponseWithImageMetadata(_) => Ok(()),
            UploadResult::Error(err) => {
                println!("CLOUDINARY: {}", err.error.message);
                return Err(err.error.message);
            }
        },
        Err(err) => {
            println!("{err}");
            return Err(err.to_string());
        }
    }
}

pub async fn execute(
    db_conn: &DatabaseConnection,
    invention: UpdateInvention,
) -> HandlerReturn<String> {
    let invention_id = Uuid::parse_str(&invention.id);

    if let Err(err) = invention_id {
        println!("{err}");

        return HandlerReturn::Failure((
            StatusCode::BAD_REQUEST,
            "ERRO AO LER ID, CONFIRA SE OS DADOS ESTÃO CORRETOS".to_string(),
        ));
    }

    let invention_id = invention_id.unwrap();

    let check_for_invention: Result<Option<invention_entity::Model>, sea_orm::DbErr> =
        invention_entity::Entity::find_by_id(invention_id)
            .one(db_conn)
            .await;

    match check_for_invention {
        Ok(checked_invention) => {
            if let Some(checked_invention) = checked_invention {
                if let Some(file) = invention.file {
                    if let Err(err) = upload_image(file, &invention_id.to_string()).await {
                        println!("{err}");

                        return HandlerReturn::Failure((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "ERRO AO SALVAR IMAGEM. TENTE NOVAMENTE MAIS TARDE.".to_string(),
                        ));
                    }
                }

                let mut checked_invention: invention_entity::ActiveModel = checked_invention.into();

                checked_invention.id = Set(invention_id.to_owned());
                checked_invention.name = Set(invention.name.to_owned());
                checked_invention.text = Set(invention.text.to_owned());
                checked_invention.external_link = Set(invention.external_link.to_owned());
                checked_invention.lat = Set(invention.lat.to_owned());
                checked_invention.lon = Set(invention.lon.to_owned());

                let result = invention_entity::Entity::update(checked_invention.clone())
                    .exec(db_conn)
                    .await;

                if let Err(err) = result {
                    println!("{err}");

                    return HandlerReturn::Failure((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "ERRO AO ATUALIZAR NO BANDO DE DADOS. TENTE NOVAMENTE MAIS TARDE."
                            .to_string(),
                    ));
                } else {
                    return HandlerReturn::Success((
                        StatusCode::OK,
                        "INVENÇÃO EDITADA COM SUCESSO".to_string(),
                    ));
                }
            } else {
                return HandlerReturn::Failure((
                    StatusCode::NOT_FOUND,
                    "INVENÇÃO NÃO ENCONTRADA NO BANCO DE DADOS".to_string(),
                ));
            }
        }

        Err(err) => {
            println!("{err}");

            return HandlerReturn::Failure((
                StatusCode::INTERNAL_SERVER_ERROR,
                "ERRO AO BUSCAR INVENÇÃO NO BANDO DE DADOS. TENTE NOVAMENTE MAIS TARDE."
                    .to_string(),
            ));
        }
    }
}
