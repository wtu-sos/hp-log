extern crate hp_log;
use std::path::PathBuf;

use hp_log::*;

fn main() {
    Logger::load_config(PathBuf::from("./"));
    info!("main running");
    debug!("main running");
    warn!("main running");
    error!("main running");
    fatal!("main running");
    Logger::close();
}
