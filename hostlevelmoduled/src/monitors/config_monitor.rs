
use std::time::Duration;
use anyhow::{Error, Result};

use azure_iot_mqtt::{
    module::Client,
    Transport::Tcp,
};

const TWIN_CONFIG_MAX_BACK_OFF: Duration = Duration::from_secs(30);
const TWIN_CONFIG_KEEP_ALIVE: Duration = Duration::from_secs(300);

pub fn get_sdk_client() -> Result<Client, Error> {
    let client = match Client::new_for_edge_module(
        Tcp,
        None,
        TWIN_CONFIG_MAX_BACK_OFF,
        TWIN_CONFIG_KEEP_ALIVE,
    ) {
        Ok(client) => client,
        Err(err) => return Err(anyhow::anyhow!("Could not create client: {}", err)),
    };

    Ok(client)
}