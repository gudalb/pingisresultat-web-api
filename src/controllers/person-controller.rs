use axum::{Router, routing::get};
use crate::service::Service;

pub struct PersonController {

}

impl PersonController {
    pub fn get_routes() -> Router {
        let routes = Router::new()
            .route("/test", get(Service::get_stuff).post(Service::post_stuff));
        routes
    }
}