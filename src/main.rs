use axum::{response::Html, routing::get, Router};
use config::Config;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::logger::logger_init;

mod config;
mod error;
mod logger;

#[tokio::main]
async fn main() {
    let config = Config::build().expect("Init config failed");

    logger_init(&config).expect("cannot init logger");

    let app = Router::new().route("/", get(handler)).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    axum::Server::bind(&format!("0.0.0.0:{}", config.port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello world</h1>")
}
