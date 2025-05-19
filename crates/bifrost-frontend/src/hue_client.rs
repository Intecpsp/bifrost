use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;

use hue::api::{GroupedLightUpdate, LightUpdate, SceneUpdate};

#[derive(Clone)]
pub struct HueClient {
    client: reqwest::Client,
    url: Url,
}

#[derive(Error, Debug)]
pub enum HueClientError {
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error("Error: {0:?}")]
    HueError(Vec<String>),
}

pub type HueClientResult<T> = Result<T, HueClientError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct V2Reply<T> {
    pub data: Vec<T>,
    pub errors: Vec<String>,
}

impl HueClient {
    #[must_use]
    pub const fn new(client: reqwest::Client, url: Url) -> Self {
        Self { client, url }
    }

    #[must_use]
    pub fn from_url(url: Url) -> Self {
        Self::new(reqwest::Client::new(), url)
    }

    pub async fn get<T: DeserializeOwned>(&self, scope: &str) -> HueClientResult<T> {
        let url = self.url.join(scope)?;

        Ok(self.client.get(url).send().await?.json().await?)
    }

    pub async fn post<I: Serialize, O: DeserializeOwned>(
        &self,
        scope: &str,
        data: I,
    ) -> HueClientResult<O> {
        let url = self.url.join(scope)?;

        Ok(self
            .client
            .post(url)
            .json(&data)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn put<I: Serialize, O: DeserializeOwned>(
        &self,
        scope: &str,
        data: I,
    ) -> HueClientResult<O> {
        let url = self.url.join(scope)?;

        Ok(self
            .client
            .put(url)
            .json(&data)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn clip_put<I: Serialize, O: DeserializeOwned>(
        &self,
        scope: &str,
        data: I,
    ) -> HueClientResult<Vec<O>> {
        let v2reply: V2Reply<O> = self
            .put(&format!("/clip/v2/resource/{scope}"), data)
            .await?;
        if v2reply.errors.is_empty() {
            Ok(v2reply.data)
        } else {
            Err(HueClientError::HueError(v2reply.errors))
        }
    }
}

impl HueClient {
    pub async fn light_update(&self, id: Uuid, upd: LightUpdate) -> HueClientResult<()> {
        self.clip_put::<_, Value>(&format!("light/{id}"), upd)
            .await?;
        Ok(())
    }

    pub async fn grouped_light_update(
        &self,
        id: Uuid,
        upd: GroupedLightUpdate,
    ) -> HueClientResult<()> {
        self.clip_put::<_, Value>(&format!("grouped_light/{id}"), upd)
            .await?;
        Ok(())
    }

    pub async fn scene_update(&self, id: Uuid, upd: SceneUpdate) -> HueClientResult<()> {
        self.clip_put::<_, Value>(&format!("scene/{id}"), upd)
            .await?;
        Ok(())
    }
}
