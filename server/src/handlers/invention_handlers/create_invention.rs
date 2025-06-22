use actix_multipart::form::tempfile::TempFile;
use actix_web::http::StatusCode;
use cloudinary::upload::{OptionalParameters, Source, Upload, UploadResult};
use sea_orm::ActiveModelTrait;
use sea_orm::{ActiveValue::Set, DatabaseConnection};
use std::collections::BTreeSet;
use uuid::Uuid;

use entity::invention as invention_entity;

use crate::env::ENV;
use crate::handlers::HandlerReturn;
use crate::models::invention::{CreateInvention, Invention};

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
    invention: CreateInvention,
) -> HandlerReturn<Invention> {
    let invention_id = Uuid::new_v4();

    if let Err(err) = upload_image(invention.file, &invention_id.to_string()).await {
        println!("{err}");

        return HandlerReturn::Failure((
            StatusCode::INTERNAL_SERVER_ERROR,
            "ERRO AO SALVAR IMAGEM. TENTE NOVAMENTE MAIS TARDE.".to_string(),
        ));
    }

    let new_invention = invention_entity::ActiveModel {
        id: Set(invention_id.to_owned()),
        name: Set(invention.name.to_owned()),
        text: Set(invention.text.to_owned()),
        external_link: Set(invention.external_link.to_owned()),
        lat: Set(invention.lat.to_owned()),
        lon: Set(invention.lon.to_owned()),
        ..Default::default()
    };

    let new_invention = new_invention.insert(db_conn).await;

    match new_invention {
        Ok(invention_model) => {
            let invention = Invention {
                id: invention_model.id.to_owned(),
                name: invention_model.name.to_owned(),
                text: invention_model.text.to_owned(),
                external_link: invention_model.external_link.to_owned(),
                lat: invention_model.lat.to_owned(),
                lon: invention_model.lon.to_owned(),
            };

            return HandlerReturn::Success((StatusCode::CREATED, invention));
        }
        Err(error) => {
            println!("{error}");

            return HandlerReturn::Failure((
                StatusCode::INTERNAL_SERVER_ERROR,
                "ERRO AO SALVAR NO BANDO DE DADOS. TENTE NOVAMENTE MAIS TARDE.".to_string(),
            ));
        }
    }
}
