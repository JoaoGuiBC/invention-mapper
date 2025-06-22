pub mod create_invention;
pub mod create_invention_with_wiki;
pub mod delete_invention;
pub mod list_inventions;
pub mod update_invention;

use actix_web::web;

pub const INVENTION_SWAGGER_TAG: &str = "Invention related routes";

pub fn init_invention_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/invention")
            .service(create_invention::route)
            .service(create_invention_with_wiki::route)
            .service(delete_invention::route)
            .service(list_inventions::route)
            .service(update_invention::route),
    );
}
