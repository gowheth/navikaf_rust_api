/*
    Function to
 */

use crate::utils::setup_logger;

mod utils;

fn main() {
    setup_logger(true, Some("info"));
    log::info!("Hello, world!");
}
