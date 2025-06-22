use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Invention {
    pub id: Uuid,
    pub name: String,
    pub text: String,
    pub external_link: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug, MultipartForm, ToSchema)]
pub struct CreateInvention {
    #[multipart(limit = "100MB")]
    #[schema(value_type = String, format = Binary)]
    pub file: TempFile,
    #[schema(value_type = String)]
    pub name: Text<String>,
    #[schema(value_type = String)]
    pub text: Text<String>,
    #[schema(value_type = String)]
    pub external_link: Text<String>,
    #[schema(value_type = f32)]
    pub lat: Text<f32>,
    #[schema(value_type = f32)]
    pub lon: Text<f32>,
}

#[derive(Debug, MultipartForm, ToSchema)]
pub struct CreateInventionWithWiki {
    #[multipart(limit = "100MB")]
    #[schema(value_type = String)]
    pub wikipedia_link: Text<String>,
    #[schema(value_type = f32)]
    pub lat: Text<f32>,
    #[schema(value_type = f32)]
    pub lon: Text<f32>,
}

#[derive(Debug, MultipartForm, ToSchema)]
pub struct UpdateInvention {
    #[schema(value_type = String)]
    pub id: Text<String>,
    #[multipart(limit = "100MB")]
    #[schema(value_type = String, format = Binary)]
    pub file: Option<TempFile>,
    #[schema(value_type = String)]
    pub name: Text<String>,
    #[schema(value_type = String)]
    pub text: Text<String>,
    #[schema(value_type = String)]
    pub external_link: Text<String>,
    #[schema(value_type = f32)]
    pub lat: Text<f32>,
    #[schema(value_type = f32)]
    pub lon: Text<f32>,
}
