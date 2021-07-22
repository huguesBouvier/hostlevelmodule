use identity_client::{IdentityClient};
use url::Url;
use identity_client_wait;

use hostlevelmoduled::monitors::config_monitor;

const MODULE_NAME: &str = "host_level";

#[tokio::main]
async fn main() {
    let url = Url::parse("unix:///run/aziot/identityd.sock").expect("Url parse failed");
    let id_client = IdentityClient::new(aziot_identity_common_http::ApiVersion::V2020_09_01, &url);
    let identity = identity_client_wait::get_device(&id_client).unwrap();
    
    let _ = identity_client_wait::create_module(&id_client, MODULE_NAME);
    let _module = identity_client_wait::get_module(&id_client, MODULE_NAME).unwrap();

    let _hostname = match identity {
        aziot_identity_common::Identity::Aziot(aziot_id) => Ok(aziot_id.gateway_host),
        aziot_identity_common::Identity::Local(_) => Err(()),
    }.unwrap();

    let trust_bundle = identity_client_wait::get_trust_bundle(&id_client);
    println!("{:?}", trust_bundle);

/*     let client = config_monitor::get_sdk_client().unwrap();
    let mut shutdown_sdk = client
        .inner()
        .shutdown_handle()
        .unwrap();

    shutdown_sdk
        .shutdown()
        .await
        .unwrap();  */  
}