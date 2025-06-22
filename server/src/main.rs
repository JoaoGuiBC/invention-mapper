mod env;
mod handlers;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use sea_orm::Database;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::env::ENV;
use crate::routes::{ApiDoc, init};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_conn = Database::connect(ENV.db_string.clone())
        .await
        .expect("Failed to connect to database");

    println!(
        "HTTP server running at http://{}:{}",
        ENV.server_host, ENV.server_port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(db_conn.clone()))
            .service(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
            .configure(init)
    })
    .bind(format!("{}:{}", ENV.server_host, ENV.server_port))?
    .run()
    .await
}
