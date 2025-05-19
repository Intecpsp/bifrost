use std::collections::VecDeque;
use std::sync::Arc;

use log::{Log, Metadata, Record};
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::sync::{RwLock, RwLockReadGuard};

use bifrost_api::logging::LogRecord;

use crate::error::ApiResult;

pub struct LogTap {
    inner: Box<dyn Log>,
    channel: Sender<LogRecord>,
}

impl Log for LogTap {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        let logrec = LogRecord::from(record);
        let _ = self.channel.send(logrec);

        self.inner.log(record);
    }

    fn flush(&self) {
        self.inner.flush();
    }
}

impl LogTap {
    const DEFAULT_CAPACITY: usize = 1024;

    pub fn init(inner: impl Log + 'static) -> ApiResult<Receiver<LogRecord>> {
        Self::init_with_capacity(inner, Self::DEFAULT_CAPACITY)
    }

    pub fn init_with_capacity(
        inner: impl Log + 'static,
        capacity: usize,
    ) -> ApiResult<Receiver<LogRecord>> {
        let (tx, rx) = broadcast::channel(capacity);

        let res = Box::new(Self {
            inner: Box::new(inner),
            channel: tx,
        });

        log::set_max_level(log::LevelFilter::Debug);
        log::set_boxed_logger(res)?;

        Ok(rx)
    }
}

pub struct LogHistory {
    tap: Receiver<LogRecord>,
    history: Arc<RwLock<VecDeque<LogRecord>>>,
}

impl LogHistory {
    const DEFAULT_CAPACITY: usize = 1024;

    #[must_use]
    pub fn new(tap: Receiver<LogRecord>) -> Self {
        Self::new_with_capacity(tap, Self::DEFAULT_CAPACITY)
    }

    #[must_use]
    pub fn new_with_capacity(tap: Receiver<LogRecord>, capacity: usize) -> Self {
        let history = Arc::new(RwLock::new(VecDeque::with_capacity(capacity)));

        let logtap = Self::tap_logs(tap.resubscribe(), history.clone());

        tokio::spawn(logtap);

        Self { tap, history }
    }

    async fn tap_logs(mut channel: Receiver<LogRecord>, history: Arc<RwLock<VecDeque<LogRecord>>>) {
        while let Ok(evt) = channel.recv().await {
            let mut lock = history.write().await;
            if lock.len() == lock.capacity() {
                lock.pop_front();
            }

            lock.push_back(evt);
        }
    }

    #[must_use]
    pub fn subscribe(&self) -> Receiver<LogRecord> {
        self.tap.resubscribe()
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, VecDeque<LogRecord>> {
        self.history.read().await
    }
}
