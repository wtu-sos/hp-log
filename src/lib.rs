//extern crate serde;
 #![feature(rustc_private)]
#[macro_use]
extern crate serde_derive;
extern crate log;

extern crate toml;

#[cfg(windows)]
extern crate wincolor;

#[macro_use]
extern crate lazy_static;

mod event;
mod config;
mod writer;
mod appender;
mod color;

pub mod filter;
pub mod logger;

use std::fmt::Arguments;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

pub use crate::{
    logger::{ThreadLocalLogger, Logger, SendEvent},
    event::Event,
    filter::FilterLevel,
};

thread_local! {
    pub static LOG_SENDER: ThreadLocalLogger = ThreadLocalLogger::new();
}

pub fn init(path: String) -> Result<(), log::SetLoggerError> {
    config::Config::create_instance(Some(PathBuf::from(path)));

    log::set_boxed_logger(CombineLogger::new())?;
    let filter = config::Config::instance().global_max_level();
    let level = log::LevelFilter::from_str(filter.as_str()).unwrap_or(log::LevelFilter::Trace);
    log::set_max_level(level);
    
    Ok(())
}

pub fn close() {
    Logger::close();
}

pub struct CombineLogger {
    file_filter: filter::Filters,
    console_filter: filter::Filters,
}

impl CombineLogger {
    pub fn new() -> Box<Self> {
        let console_conf = config::Config::instance().console_conf();
        let file_conf = config::Config::instance().file_conf();
        let logger = CombineLogger {
            file_filter: filter::Filters::new(&file_conf),
            console_filter: filter::Filters::new(&console_conf),
        };

        Box::new(logger)
    }
}

impl log::Log for CombineLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let filter = metadata.level().into();
        if self.file_filter.is_pass(filter) || self.console_filter.is_pass(filter) {
            return true
        }

        false
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            send_record(record);
        }
    }

    fn flush(&self) {
        let _ = std::io::stdout().flush();
    }
}

#[allow(unused)]
pub fn send_event(level: FilterLevel, file: &'static str, line: u32, msg: Arguments) {
    LOG_SENDER.with(|s| {
        let ev = Event::new(level, s.get_thread_tag(), file, line, msg);
        s.get_sender().send_event(ev);
    });
}

#[allow(unused)]
pub fn send_record(record: &log::Record) {
    LOG_SENDER.with(|s| {
        let ev = Event::from_record(s.get_thread_tag(), record);
        s.get_sender().send_event(ev);
    });
}

mod test {

    #[test]
    fn multi_test() {
        use std::time::{Duration};
        use std::thread;
        use std::path::PathBuf;
        //use std::io;
        use crate::{Logger, init};

        init();
        Logger::load_config(PathBuf::from("./"));

        let mut ths = Vec::new();

        let max_format_count = 50_0000;
        let t1 = max_format_count.clone(); 

        println!("mutlt test");
        //let m_now = Instant::now();
        for _i in 0..8 {
            ths.push(thread::spawn(
                    move || {
                        //let d_now = Instant::now();
                        for idx in 0..t1 {
                            warn!("199999999999999-=dfghjkl;'kald;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m .... {}", 1,2,3, idx);
                            debug!("1234567890-=dfghjkl;'kald;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m .... {}", 1,2,3, idx);
                            error!("1234567890-=dfghjkl;'kald;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m ********* {}", 1,2,3, idx);
                            info!("1234567890-=dfghjkl;'ka0000000000;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m ********* {}", 1,2,3, idx);
                            trace!("1234567890-=dfghjkl;'ka0000000000;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m ********* {}", 1,2,3, idx);
                        }
                    }));
        }
        thread::sleep(Duration::from_secs(2));
    }
}
