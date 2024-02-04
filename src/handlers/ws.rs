use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path,
    },
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use uuid::Uuid;

use crate::{
    auth::AuthSession,
    models::extension_web_socket::{ExtensionWebSocketError, ExtensionWebSocketMatch},
};
use futures::{sink::SinkExt, stream::StreamExt};

pub async fn ws_handler(
    Extension(web_socket): Extension<ExtensionWebSocketMatch>,
    Path(_game_name): Path<String>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket_match(socket, web_socket))
}

pub async fn ws_handler_error(
    auth_session: AuthSession,
    Extension(web_socket): Extension<ExtensionWebSocketError>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    ws.on_upgrade(move |socket| handle_socket_error(socket, web_socket, user.id))
}

pub async fn handle_socket_error(
    ws: WebSocket,
    web_socket: ExtensionWebSocketError,
    user_id: Uuid,
) {
    let cloned_server_rx = web_socket.rx.clone();
    let (mut ws, mut ws_recv) = ws.split();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = cloned_server_rx.recv_async().await {
            if user_id == msg.app_user_id && ws.send(Message::Text(msg.message)).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task =
        tokio::spawn(async move { while let Some(Ok(_)) = ws_recv.next().await {} });

    // If either task exits, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}

pub async fn handle_socket_match(ws: WebSocket, web_socket: ExtensionWebSocketMatch) {
    let cloned_server_rx = web_socket.rx.clone();
    let (mut ws, mut ws_recv) = ws.split();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = cloned_server_rx.recv_async().await {
            if ws.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task =
        tokio::spawn(async move { while let Some(Ok(_)) = ws_recv.next().await {} });

    // If either task exits, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}
