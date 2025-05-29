use std::pin::Pin;
use std::task::{Context, Poll};

use futures::{SinkExt, Stream};
use serde::Serialize;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{self, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use wled::{StateSegUpdate, StateUpdate};

use crate::error::ApiResult;

pub struct WledWebSocket {
    pub name: String,
    pub socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WledWebSocket {
    pub const fn new(name: String, socket: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { name, socket }
    }

    pub async fn send(&mut self, payload: impl Serialize) -> ApiResult<()> {
        let json = serde_json::to_string(&payload)?;
        log::warn!("WLED: {json}");
        let msg = Message::text(json);
        Ok(self.socket.send(msg).await?)
    }

    pub async fn send_group_state(&mut self, state: StateUpdate) -> ApiResult<()> {
        self.send(json!(state)).await
    }

    pub async fn send_state(&mut self, state: StateSegUpdate) -> ApiResult<()> {
        self.send_states(&[state]).await
    }

    pub async fn send_states(&mut self, states: &[StateSegUpdate]) -> ApiResult<()> {
        self.send(json!({"seg": states})).await
    }

    pub async fn send_preset_recall(&mut self, index: u8) -> ApiResult<()> {
        self.send(json!({"ps": index})).await
    }

    pub async fn send_preset_store(&mut self, index: u8) -> ApiResult<()> {
        self.send(json!({"psave": index})).await
    }
}

impl Stream for WledWebSocket
where
    Self: Unpin,
{
    type Item = Result<Message, tungstenite::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        WebSocketStream::poll_next(Pin::new(&mut self.socket), cx)
    }
}
