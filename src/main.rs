/*
    Function to
 */

use crate::utils::setup_logger;
use crate::config::config_utils;
use crate::model::kafka_client::ClientConfigRequest;

mod utils;
mod config;
mod model;

fn main() {
    setup_logger(true, Some("info"));

    let config_request = ClientConfigRequest {
        client_id: "1231lkjdsf".to_string(),
        bootstrap_servers: "localhost:9092".to_string()
    };

    let client_config = config_utils::nv_create_config(&config_request);
    log::info!("Client Config: {:?}", client_config);

}


