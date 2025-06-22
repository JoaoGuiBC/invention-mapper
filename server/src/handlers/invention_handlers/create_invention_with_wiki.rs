use actix_web::http::StatusCode;
use cloudinary::upload::{OptionalParameters, Source, Upload, UploadResult};
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use reqwest::Url;
use sea_orm::ActiveModelTrait;
use sea_orm::{ActiveValue::Set, DatabaseConnection};
use serde::Deserialize;
use std::collections::BTreeSet;
use uuid::Uuid;

use entity::invention as invention_entity;

use crate::env::ENV;
use crate::handlers::HandlerReturn;
use crate::models::invention::{CreateInventionWithWiki, Invention};

#[derive(Deserialize)]
struct ThumbnailData {
    source: String,
}

#[derive(Deserialize)]
struct PageData {
    title: String,
    extract: String,
    thumbnail: ThumbnailData,
}

#[derive(Deserialize)]
struct QueryData {
    pages: Vec<PageData>,
}

#[derive(Deserialize)]
struct WikiData {
    query: QueryData,
}

async fn get_wiki_data(article_name: String) -> Result<WikiData, Box<dyn std::error::Error>> {
    let fetch_data_url = format!(
        "https://pt.wikipedia.org/w/api.php?action=query&prop=extracts|pageimages&titles={article_name}&format=json&formatversion=2&explaintext=1"
    );

    let wiki_data = reqwest::get(fetch_data_url)
        .await?
        .json::<WikiData>()
        .await?;

    Ok(wiki_data)
}

pub async fn upload_image(image_url: String, invention_id: &String) -> Result<(), String> {
    let image_url = Url::parse(&image_url).unwrap();

    let options = BTreeSet::from([
        OptionalParameters::Folder(ENV.cloudinary_images_folder.clone()),
        OptionalParameters::PublicId(invention_id.to_owned()),
    ]);
    let upload = Upload::new(
        ENV.cloudinary_api_key.clone(),
        ENV.cloudinary_cloud_name.clone(),
        ENV.cloudinary_api_secret.clone(),
    );
    let result = upload.image(Source::Url(image_url), &options).await;

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
    invention: CreateInventionWithWiki,
) -> HandlerReturn<Invention> {
    let article = invention.wikipedia_link.find("/wiki/");

    let (name, text, image_url) = match article {
        Some(position) => {
            let url_size = invention.wikipedia_link.len();

            let article_name: String = invention
                .wikipedia_link
                .chars()
                .skip(position + 6)
                .take(url_size - position)
                .collect();

            let wiki_data = get_wiki_data(article_name).await;

            match wiki_data {
                Ok(wiki_data) => (
                    wiki_data.query.pages[0].title.to_owned(),
                    wiki_data.query.pages[0].extract.to_owned(),
                    wiki_data.query.pages[0].thumbnail.source.to_owned(),
                ),
                Err(error) => {
                    println!("{error}");

                    return HandlerReturn::Failure((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "FALHA AO CARREGAR DADOS DA WIKIPÉDIA. TENTE NOVAMENTE MAIS TARDE."
                            .to_string(),
                    ));
                }
            }
        }
        None => {
            return HandlerReturn::Failure((
                StatusCode::BAD_REQUEST,
                "URL DA WIKIPÉDIA INVÁLIDA.".to_string(),
            ));
        }
    };

    let ollama = Ollama::default();

    let prompt = format!("{}{}", ENV.ollama_prompt, text);
    let ollama_result = ollama
        .generate(GenerationRequest::new(ENV.ollama_model.clone(), prompt))
        .await;

    match ollama_result {
        Ok(result) => {
            let invention_id = Uuid::new_v4();

            if let Err(err) = upload_image(image_url, &invention_id.to_string()).await {
                println!("{err}");

                return HandlerReturn::Failure((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "ERRO AO SALVAR IMAGEM. TENTE NOVAMENTE MAIS TARDE.".to_string(),
                ));
            }

            let new_invention = invention_entity::ActiveModel {
                id: Set(invention_id.to_owned()),
                name: Set(name.to_owned()),
                text: Set(result.response.to_owned()),
                external_link: Set(invention.wikipedia_link.to_owned()),
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
        Err(error) => {
            println!("{error}");

            return HandlerReturn::Failure((StatusCode::INTERNAL_SERVER_ERROR, error.to_string()));
        }
    }
}
