use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path,
    },
    response::IntoResponse,
    Extension,
};

use crate::models::extension_web_socket::ExtensionWebSocket;

pub async fn ws_handler(
    Extension(web_socket): Extension<ExtensionWebSocket>,
    Path(game_name): Path<String>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, game_name, web_socket))
}

pub async fn handle_socket(mut ws: WebSocket, _game_name: String, web_socket: ExtensionWebSocket) {
    let cloned_server_rx = web_socket.rx.clone();

    while let Ok(msg) = cloned_server_rx.recv_async().await {
        ws.send(Message::Text(msg.to_string())).await.unwrap();
    }
}
