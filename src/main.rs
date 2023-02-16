#[path="models/human.rs"] mod human;
#[path="handlers/services.rs"] mod service;
#[path="controllers/person-controller.rs"] mod person_controller;

use dotenv::dotenv;
use person_controller::PersonController;
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .merge(PersonController::get_routes());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
