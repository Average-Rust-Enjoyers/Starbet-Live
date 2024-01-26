use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path,
    },
    response::IntoResponse,
    Extension,
};

use crate::models::extension_web_socket::ExtensionWebSocket;
use futures::{sink::SinkExt, stream::StreamExt};

pub async fn ws_handler(
    Extension(web_socket): Extension<ExtensionWebSocket>,
    Path(game_name): Path<String>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, game_name, web_socket))
}

pub async fn handle_socket(ws: WebSocket, _game_name: String, web_socket: ExtensionWebSocket) {
    let cloned_server_rx = web_socket.rx.clone();
    let (mut ws, mut ws_recv) = ws.split();
    
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = cloned_server_rx.recv_async().await {
            if ws.send(Message::Text(msg.to_string())).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(_)) = ws_recv.next().await {}
    });

    // If either task exits, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

}
