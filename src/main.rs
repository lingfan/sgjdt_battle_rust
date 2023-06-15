use axum::{routing::get, Router};
use std::net::SocketAddr;

pub mod common;
pub mod handlers;
pub mod model;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handlers::user::handler))
        .route("/login/:username/:password", get(handlers::user::login))
        .route("/server_list/:token", get(handlers::user::server_list));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
