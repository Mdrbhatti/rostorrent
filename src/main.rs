mod logger;

use log::{info};

fn main() {
    logger::init_logger().expect("Error while creating logger");
    info!("Starting rostorrent");
}
