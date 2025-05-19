use std::sync::Arc;
use std::time::Duration;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::Response;
use bifrost_api::config::AppConfig;
use bifrost_api::logging::LogRecord;
use bytes::Bytes;
use tokio::select;

use bifrost_api::backend::BackendRequest;
use bifrost_api::service::Service;
use bifrost_api::websocket::Update;
use hue::event::EventBlock;
use svc::manager::{ServiceEvent, SvmClient};

use crate::routes::bifrost::BifrostApiResult;
use crate::server::appstate::AppState;
use crate::server::hueevents::HueEventRecord;

struct WebSocketTask {
    state: AppState,
    ws: WebSocket,
    mgr: SvmClient,
}

#[allow(clippy::unnecessary_wraps, clippy::unused_self)]
impl WebSocketTask {
    const KEEP_ALIVE: Duration = Duration::from_secs(2);

    pub fn new(state: AppState, ws: WebSocket) -> Self {
        let mgr = state.manager();

        Self { state, ws, mgr }
    }

    async fn send(&mut self, value: Update) -> BifrostApiResult<()> {
        let text = serde_json::to_string(&value)?.into();

        self.ws.send(Message::Text(text)).await?;

        Ok(())
    }

    async fn send_ping(&mut self) -> BifrostApiResult<Option<Update>> {
        // Sending pings allow us to detect dead connections
        self.ws.send(Message::Ping(Bytes::new())).await?;

        Ok(None)
    }

    fn handle_websocket_message(&self, msg: &Message) -> BifrostApiResult<Option<Update>> {
        log::trace!("Websocket message: {msg:?}");
        Ok(None)
    }

    fn handle_backend_event(
        &self,
        backend_event: &Arc<BackendRequest>,
    ) -> BifrostApiResult<Option<Update>> {
        log::info!("Backend event: {backend_event:?}");
        Ok(Some(Update::BackendRequest((**backend_event).clone())))
    }

    fn handle_hue_event(&self, hue_event: HueEventRecord) -> BifrostApiResult<Option<Update>> {
        log::info!("Hue event: {hue_event:?}");
        Ok(Some(Update::HueEvent(hue_event.block)))
    }

    fn handle_app_config(&self, app_config: AppConfig) -> BifrostApiResult<Option<Update>> {
        log::info!("Config: {app_config:?}");
        Ok(Some(Update::AppConfig(app_config)))
    }

    async fn handle_service_event(
        &mut self,
        service_event: Option<ServiceEvent>,
    ) -> BifrostApiResult<Option<Update>> {
        let Some(service_event) = service_event else {
            log::error!("service event channel broke");
            panic!();
        };

        log::trace!("service event: {service_event:?}");

        let name = self.mgr.lookup_name(service_event.id()).await?;

        let service = Service {
            id: service_event.id(),
            name,
            state: service_event.state(),
        };

        Ok(Some(Update::ServiceUpdate(service)))
    }

    const fn handle_log_event(&self, log_event: LogRecord) -> BifrostApiResult<Option<Update>> {
        Ok(Some(Update::LogEvent(log_event)))
    }

    async fn handle_socket(mut self) -> BifrostApiResult<()> {
        let lock = self.state.res.lock().await;
        let mut backend_events = lock.backend_event_stream();
        let mut hue_events = lock.hue_event_stream().subscribe();
        let hue_state = lock.get_resources();
        drop(lock);

        let mut config = self.state.config_subscribe();

        let mut svc_events = self.mgr.subscribe().await?.1;
        let mut log_events = self.state.logger();

        self.send(Update::AppConfig((*self.state.config()).clone()))
            .await?;

        self.send(Update::HueEvent(EventBlock::add(hue_state)))
            .await?;

        let log = self.state.log.clone();
        for evt in &*log.read().await {
            self.send(Update::LogEvent(evt.clone())).await?;
        }

        loop {
            let reply = select! {
                () = tokio::time::sleep(Self::KEEP_ALIVE) => self.send_ping().await,
                _ = config.changed() => self.handle_app_config(config.borrow().clone()),
                Some(msg) = self.ws.recv() => self.handle_websocket_message(&msg?),
                backend_event = backend_events.recv() => self.handle_backend_event(&backend_event?),
                service_event = svc_events.recv() => self.handle_service_event(service_event).await,
                hue_event = hue_events.recv() => self.handle_hue_event(hue_event?),
                log_event = log_events.recv() => self.handle_log_event(log_event?),
            };

            if let Some(reply) = reply? {
                self.send(reply).await?;
            }
        }
    }
}

pub async fn websocket(State(state): State<AppState>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|ws| async move {
        let wst = WebSocketTask::new(state, ws);

        match wst.handle_socket().await {
            Ok(()) => {
                log::info!("Websocket closed.");
            }
            Err(err) => {
                log::error!("Websocket error: {err:?}");
            }
        }
    })
}
