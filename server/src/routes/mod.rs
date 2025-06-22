mod invention_routes;

use actix_web::{HttpResponse, web};
use utoipa::OpenApi;

use invention_routes::{
    INVENTION_SWAGGER_TAG, create_invention, create_invention_with_wiki, delete_invention,
    init_invention_routes, list_inventions, update_invention,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title="INVENTION MAPPER BACKEND"
    ),
    tags((name = INVENTION_SWAGGER_TAG)),
    paths(
        create_invention::route,
        create_invention_with_wiki::route,
        delete_invention::route,
        list_inventions::route,
        update_invention::route
    )
)]
pub struct ApiDoc;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("API is running")
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .configure(init_invention_routes),
    );
}
