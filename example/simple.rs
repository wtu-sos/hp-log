//#![feature(rustc_private)]
extern crate hp_log;
#[macro_use]
extern crate log;

fn main() {
    hp_log::init("./".to_string());

    log_enabled!(log::Level::Debug);
    log_enabled!(log::Level::Trace);

    trace!("main running ````````````````````");
    info!("main running info");
    debug!("main running .........................");
    warn!("main running ****************");
    error!("main running +++++++++++++++++++++++++");

    hp_log::close();
}
