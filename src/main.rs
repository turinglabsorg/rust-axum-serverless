use lambda_http::{
    run,
    Error,
};
use axum::{
    response::Json,
    Router,
    routing::{get},
};
use serde_json::{Value, json};

async fn root() -> Json<Value> {
    Json(json!({ "msg": "🦀", "error": false }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let app = Router::new()
        .route("/", get(root));

    run(app).await
}