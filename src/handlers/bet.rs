use crate::{
    models::{extension_web_socket::ExtensionWebSocket, game_match::GameMatchGetById},
    repositories::game_match::GameMatchRepository,
    templates::PlaceBetForm,
};

use crate::common::repository::DbReadOne;
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use uuid::Uuid;

pub async fn place_bet_handler(
    Extension(web_socket): Extension<ExtensionWebSocket>,
) -> impl IntoResponse {
    web_socket.tx.send_async("bet".to_string()).await.unwrap();

    StatusCode::OK
}

pub async fn get_bet_handler(
    Extension(mut game_match_repo): Extension<GameMatchRepository>,
    Path((match_id, prediction)): Path<(String, String)>,
) -> impl IntoResponse {
    let match_id = Uuid::parse_str(&match_id).unwrap();
    let game_match = game_match_repo
        .read_one(&GameMatchGetById { id: match_id })
        .await
        .unwrap();

    let predicted_team = match prediction.as_str() {
        "a" => game_match.name_a,
        _ => game_match.name_b,
    };

    let template = PlaceBetForm {
        match_id: game_match.id,
        predicted_team,
    }
    .render()
    .unwrap();

    (StatusCode::OK, Html(template).into_response())
}
