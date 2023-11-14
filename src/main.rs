use axum::{response::Json, routing::get, Router};
use lambda_http::{run, Error};
use serde_json::{json, Value};

async fn root() -> Json<Value> {
    Json(json!({ "msg": "🦀", "error": false }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();
    let app = Router::new().route("/", get(root));
    run(app).await
}
