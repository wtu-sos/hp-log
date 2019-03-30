#![feature(rustc_private)]
extern crate hp_log;
#[macro_use]
extern crate log;
use std::path::PathBuf;

use hp_log::*;

fn main() {
    Logger::load_config(PathBuf::from("./"));
    hp_log::init();
    info!("main running");
    debug!("main running");
    warn!("main running");
    error!("main running");
    trace!("main running");
    Logger::close();
}
