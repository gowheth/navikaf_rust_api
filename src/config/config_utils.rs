use rdkafka::ClientConfig;
use crate::model::kafka_client::ClientConfigRequest;

pub fn create_config(config_request : &ClientConfigRequest) -> ClientConfig {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", config_request.bootstrap_servers.as_str());

    log::info!("Config created for client id: {}", config_request.client_id);

    config
}
