/*
    Function to
 */

use crate::utils::setup_logger;
use crate::config::config_utils;
mod utils;
mod config;

fn main() {
    setup_logger(true, Some("info"));
    log::info!("Hello, world!");

    config_utils::create_config();

}


