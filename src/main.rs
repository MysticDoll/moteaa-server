use axum::{extract::Path, http::StatusCode, response::{Html, IntoResponse, Response}, routing::get, Router};
use rand::seq::SliceRandom;
use crate::motemen::*;

mod motemen;

#[tokio::main]
async fn main() {
    let app = Router::<()>::new()
        .route("/motemen/:format", get(get_motemen))
        .route("/random/:format", get(get_random_motemen))
        .route("/random/:format/:size", get(get_random_scaled_motemen))
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

async fn get_random_motemen(
    Path(format): Path<String>
) -> Response {
    let mut random_motemen = crate::motemen::MOTEMEN.clone();
    let mut rng = rand::thread_rng();
    random_motemen.as_flattened_mut().shuffle(&mut rng);

    let content: MotemenVariant = (format, random_motemen).into();
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

async fn get_random_scaled_motemen(
    Path((format, size)): Path<(String, u8)>
) -> Response {
    let mut random_motemen = crate::motemen::MOTEMEN.clone();
    let mut rng = rand::thread_rng();
    random_motemen.as_flattened_mut().shuffle(&mut rng);

    let content: MotemenVariant = (format, random_motemen, size).into();
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
