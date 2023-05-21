use askama::Template;
use axum::{
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

pub async fn page() -> impl IntoResponse {
    let template = IndexTemplate {};
    let reply_html = template.render().unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(header::CACHE_CONTROL, "max-age=600".parse().unwrap());

    (StatusCode::OK, headers, Html(reply_html).into_response())
}
