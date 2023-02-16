use std::env;
use diesel::{PgConnection, Connection};
use dotenv::dotenv;
mod controllers;
mod services;
mod schema;
mod models;
use controllers::matches_controller as MatchesController;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    establish_database_connection();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" })
        .merge(MatchesController::get_routes()));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn establish_database_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
