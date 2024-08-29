use axum::{extract::Path, http::StatusCode, response::{Html, IntoResponse, Response}, routing::get, Router};
use crate::motemen::*;

mod motemen;

#[tokio::main]
async fn main() {
    let app = Router::<()>::new()
        .route("/motemen/:format", get(get_motemen))
        .route("/motemen/:format/:size", get(get_scaled_motemen));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_motemen(
    Path(format): Path<String>
) -> Response {
    let content: MotemenVariant = format.into();
    match content {
        MotemenVariant::HTML(content) => (
            StatusCode::OK,
            Html(
                content.content
            )
        ).into_response(),
        MotemenVariant::SHELL(content) => (
            StatusCode::OK,
            content.content
        ).into_response(),
    }
}


async fn get_scaled_motemen(
    Path(param): Path<(String, u8)>
) -> Response {
    let content: MotemenVariant = param.into();
    match content {
        MotemenVariant::HTML(content) => (
            StatusCode::OK,
            Html(
                content.content
            )
        ).into_response(),
        MotemenVariant::SHELL(content) => (
            StatusCode::OK,
            content.content
        ).into_response(),
    }
}
