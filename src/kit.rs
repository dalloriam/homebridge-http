use std::sync::Arc;

use anyhow::Result;

use crate::{
    client::{Accessory, SwitchConfig},
    switch::Switch,
    HomebridgeClient,
};

pub struct HomeKit {
    homebridge: Arc<HomebridgeClient>,
}

impl HomeKit {
    pub fn connect(homebridge_host: &str) -> Self {
        Self {
            homebridge: Arc::new(HomebridgeClient::new(homebridge_host)),
        }
    }

    pub async fn add_switch(&self, config: SwitchConfig) -> Result<Switch> {
        self.homebridge.create_switch(&config).await?;
        Ok(Switch::new(config, self.homebridge.clone()))
    }

    pub async fn get_switch(&self, accessory_id: &str) -> Result<Option<Switch>> {
        let accessory = self.homebridge.get_accessory(accessory_id).await?;
        Ok(accessory
            .map(|a| {
                if let Accessory::Switch(decl) = a {
                    Some(Switch::new(decl.config, self.homebridge.clone()))
                } else {
                    None
                }
            })
            .flatten())
    }

    pub async fn switches(&self) -> Result<Vec<Switch>> {
        let mut switches = Vec::new();
        for accessory in self.homebridge.list_accessories().await? {
            if let Accessory::Switch(declaration) = accessory {
                switches.push(Switch::new(declaration.config, self.homebridge.clone()))
            }
        }

        Ok(switches)
    }
}
