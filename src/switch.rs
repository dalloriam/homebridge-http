use std::sync::Arc;

use anyhow::Result;

use crate::client::{HomebridgeClient, SwitchConfig, SwitchState};

pub struct Switch {
    config: SwitchConfig,
    client: Arc<HomebridgeClient>,
}

impl Switch {
    pub(crate) fn new(config: SwitchConfig, client: Arc<HomebridgeClient>) -> Self {
        Self { config, client }
    }

    pub fn id(&self) -> &str {
        &self.config.id
    }

    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub async fn is_on(&self) -> Result<bool> {
        let state = self.poll().await?;
        Ok(state.on)
    }

    pub async fn set(&self, on: bool) -> Result<()> {
        self.client.set_state(self.id(), &SwitchState { on }).await
    }

    async fn poll(&self) -> Result<SwitchState> {
        Ok(self
            .client
            .get_state::<SwitchState>(&self.config.id)
            .await?)
    }

    pub async fn delete(self) -> Result<()> {
        self.client.delete_accessory(self.id()).await
    }
}
