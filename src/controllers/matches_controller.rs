use axum::{Router, routing::{get, post}};
use crate::services;

pub mod matches_controller{}

pub fn get_routes() -> Router {
    let routes = Router::new()
        .route("/matches", post(services::match_service::add_match));
    routes
}