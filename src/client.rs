use anyhow::{ensure, Result};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SwitchState {
    pub on: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SwitchConfig {
    pub id: String,
    pub name: String,
    pub on_url: Option<String>,
    pub off_url: Option<String>,
}

impl SwitchConfig {
    pub fn new<S: Into<String>, T: Into<String>>(id: S, name: T) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            on_url: None,
            off_url: None,
        }
    }

    pub fn with_on_url<S: Into<String>>(mut self, on_url: S) -> Self {
        self.on_url = Some(on_url.into());
        self
    }

    pub fn with_off_url<S: Into<String>>(mut self, off_url: S) -> Self {
        self.off_url = Some(off_url.into());
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SwitchDeclaration {
    pub state: SwitchState,
    pub config: SwitchConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Accessory {
    Switch(SwitchDeclaration),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AccessoriesResponse {
    pub accessories: Vec<Accessory>,
}

pub struct HomebridgeClient {
    host: String,
    http: reqwest::Client,
}

#[derive(Deserialize, Serialize)]
struct StateMsg<T> {
    state: T,
}

impl HomebridgeClient {
    pub fn new(host: &str) -> Self {
        let http = reqwest::Client::default();
        Self {
            host: host.to_string(),
            http,
        }
    }

    pub async fn get_accessory(&self, accessory_id: &str) -> Result<Option<Accessory>> {
        let resp = self
            .http
            .get(format!("{}/accessory/{}", &self.host, accessory_id))
            .send()
            .await?;

        if resp.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        ensure!(
            resp.status().is_success(),
            "request failed: {}",
            resp.status()
        );

        let data: Accessory = serde_json::from_slice(resp.bytes().await?.as_ref())?;
        Ok(Some(data))
    }

    pub async fn list_accessories(&self) -> Result<Vec<Accessory>> {
        let resp = self
            .http
            .get(format!("{}/accessory", &self.host))
            .send()
            .await?;

        ensure!(
            resp.status().is_success(),
            "request failed: {}",
            resp.status()
        );

        let data: AccessoriesResponse = serde_json::from_slice(resp.bytes().await?.as_ref())?;
        Ok(data.accessories)
    }

    pub async fn delete_accessory(&self, accessory_id: &str) -> Result<()> {
        let resp = self
            .http
            .delete(format!("{}/accessory/{}", &self.host, accessory_id))
            .send()
            .await?;

        ensure!(
            resp.status().is_success(),
            "request failed: {}",
            resp.status()
        );

        Ok(())
    }

    pub async fn get_state<T: DeserializeOwned>(&self, accessory_id: &str) -> Result<T> {
        let resp = self
            .http
            .get(format!("{}/accessory/{}/state", &self.host, accessory_id))
            .send()
            .await?;

        ensure!(
            resp.status().is_success(),
            "request failed: {}",
            resp.status()
        );

        let data: StateMsg<T> = serde_json::from_slice(resp.bytes().await?.as_ref())?;
        Ok(data.state)
    }

    pub async fn set_state<T: Clone + Serialize>(
        &self,
        accessory_id: &str,
        state: &T,
    ) -> Result<()> {
        let resp = self
            .http
            .put(format!("{}/accessory/{}/state", &self.host, accessory_id))
            .json(&state)
            .send()
            .await?;

        ensure!(
            resp.status().is_success(),
            "request failed: {}",
            resp.status()
        );
        Ok(())
    }

    pub async fn create_switch(&self, config: &SwitchConfig) -> Result<()> {
        let resp = self
            .http
            .post(format!("{}/accessory", &self.host))
            .json(&config)
            .send()
            .await?;

        ensure!(
            resp.status().is_success(),
            "request failed: {}",
            resp.status()
        );

        Ok(())
    }
}
