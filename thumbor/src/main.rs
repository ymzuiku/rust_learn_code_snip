use axum::extract::Path;
use axum::handler;
use axum::Router;
use percent_encoding::percent_decode_str;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

mod pb;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Params {
    spec: String,
    url: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/img/:spec/:url", handler::get(generate));
    let addr = "127.0.0.1:3300".parse().unwrap();
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: pb::ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(format!("url: {}\n spec:{:#?}", url, spec))
}
