use identity_client::Error;


pub fn get_device(id_client: &identity_client::IdentityClient) -> Result<aziot_identity_common::Identity, Error>{
    let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
    runtime.block_on(id_client.get_device())
}

pub fn create_module(
    id_client: &identity_client::IdentityClient,
    module_name: &str,
)-> Result<aziot_identity_common::Identity, Error>{
    let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
    runtime.block_on(id_client.create_module(module_name))
}

pub fn get_module(
    id_client: &identity_client::IdentityClient,
    module_name: &str,
)-> Result<aziot_identity_common::Identity, Error>{
    let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
    runtime.block_on(id_client.get_module(module_name))
}

pub fn get_trust_bundle(
    id_client: &identity_client::IdentityClient,
) -> Result<aziot_identity_common_http::get_trust_bundle::Response, Error> {
    let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
    runtime.block_on(id_client.get_trust_bundle())    
}