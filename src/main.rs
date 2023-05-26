use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};

mod api;
mod page;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let config = aws_config::load_from_env().await;
    aws_sdk_dynamodb::Client::new(&config);

    let api_router = Router::new().route("/pokemon", post(api::handler::add_pokemon::add_pokemon));

    let app = Router::new()
        .route("/", get(page::index::page))
        .route("/_assets/*path", get(handle_assets))
        .nest("/api", api_router)
        .with_state(aws_sdk_dynamodb::Client::new(&config));

    run(app).await
}

static THEME_CSS: &str = include_str!("../assets/index.css");

async fn handle_assets(
    axum::extract::Path(path): axum::extract::Path<String>,
) -> impl axum::response::IntoResponse {
    let mut headers = axum::http::HeaderMap::new();

    if path == "index.css" {
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            "text/css".parse().unwrap(),
        );
        (StatusCode::OK, headers, THEME_CSS)
    } else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}
