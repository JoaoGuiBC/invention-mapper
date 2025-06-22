use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref ENV: Env = Env::get_env();
}

#[derive(Debug)]
pub struct Env {
    pub ollama_prompt: String,
    pub ollama_model: String,
    pub db_string: String,
    pub server_host: String,
    pub server_port: String,
    pub cloudinary_api_secret: String,
    pub cloudinary_api_key: String,
    pub cloudinary_cloud_name: String,
    pub cloudinary_images_folder: String,
}

impl Env {
    fn get_env() -> Self {
        dotenv().ok();
        let ollama_prompt = env::var("OLLAMA_PROMPT").expect("ENV VAR: AI PROMPT NOT DEFINED");
        let ollama_model = env::var("OLLAMA_MODEL").expect("ENV VAR: AI MODEL NOT DEFINED");
        let db_string =
            env::var("DATABASE_URL").expect("ENV VAR: DB CONNECTION STRING NOT DEFINED");
        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
        let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "3333".to_string());
        let cloudinary_api_secret =
            env::var("CLOUDINARY_API_SECRET").expect("ENV VAR: CLOUDINARY API SECRET NOT DEFINED");
        let cloudinary_api_key =
            env::var("CLOUDINARY_API_KEY").expect("ENV VAR: CLOUDINARY API KEY NOT DEFINED");
        let cloudinary_cloud_name =
            env::var("CLOUDINARY_CLOUD_NAME").expect("ENV VAR: CLOUDINARY CLOUD NAME NOT DEFINED");
        let cloudinary_images_folder =
            env::var("CLOUDINARY_IMAGES_FOLDER").unwrap_or_else(|_| "images".to_string());

        Self {
            ollama_prompt,
            ollama_model,
            db_string,
            server_host,
            server_port,
            cloudinary_api_secret,
            cloudinary_api_key,
            cloudinary_cloud_name,
            cloudinary_images_folder,
        }
    }
}
