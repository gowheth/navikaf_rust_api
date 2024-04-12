use rdkafka::admin::AdminClient;
use rdkafka::client::DefaultClientContext;
use rdkafka::ClientConfig;
use crate::model::kafka_client::ClientConfigRequest;

pub fn nv_create_config(config_request : &ClientConfigRequest) -> ClientConfig {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", config_request.bootstrap_servers.as_str());

    log::info!("Config created for client id: {}", config_request.client_id);

    config
}

pub fn nv_create_admin_client(config_request : &ClientConfigRequest) -> AdminClient<DefaultClientContext> {
    let admin_client = nv_create_config(config_request).create();
    match admin_client {
        Ok(client) => {
            log::info!("Admin client created for client id: {}", config_request.client_id);
            return client;
        },
        Err(e) => {
            log::error!("Error creating admin client: {}", e);
            panic!("Error creating admin client");
        }
    }
}
