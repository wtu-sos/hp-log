#![feature(duration_as_u128)]
//extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate lazy_static;


mod event;
mod config;
mod writer;
mod appender;

pub mod filter;
pub mod logger;

use std::fmt::Arguments;

pub use crate::{
    logger::{ThreadLocalLogger, Logger, SendEvent},
    event::Event,
    filter::FilterLevel,
};

thread_local! {
    static LOG_SENDER: ThreadLocalLogger = ThreadLocalLogger::new();
}

#[allow(unused)]
pub fn send_event(level: FilterLevel, file: &'static str, line: u32, msg: Arguments) {
    LOG_SENDER.with(|s| {
        if s.get_filter().is_pass(level) {
            let ev = Event::new(level, s.get_thread_tag(), file, line, msg);
            s.get_sender().send_event(ev);
        }
    });
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        send_event($crate::filter::FilterLevel::Debug, file!(), line!(), format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        send_event($crate::filter::FilterLevel::Info, file!(), line!(), format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        send_event($crate::filter::FilterLevel::Error, file!(), line!(), format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => {
        send_event($crate::filter::FilterLevel::Fatal, file!(), line!(), format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        send_event($crate::filter::FilterLevel::Warn, file!(), line!(), format_args!($($arg)*));
    }
}

mod test {

    #[test]
    fn multi_test() {
        //use std::time::{Duration, Instant};
        use std::thread;
        use std::path::PathBuf;
        //use std::io;
        use crate::{send_event, Logger, log_info, log_debug, log_error};

        Logger::load_config(PathBuf::from("./"));

        let mut ths = Vec::new();

        let max_format_count = 50_0000;
        let t1 = max_format_count.clone(); 

        //let m_now = Instant::now();
        for _i in 0..8 {
            ths.push(thread::spawn(
                    move || {

                        //let d_now = Instant::now();
                        for idx in 0..t1 {
                            log_debug!("1234567890-=dfghjkl;'kald;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m .... {}", 1,2,3, idx);
                            log_error!("1234567890-=dfghjkl;'kald;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m ********* {}", 1,2,3, idx);
                            log_info!("1234567890-=dfghjkl;'ka0000000000;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m ********* {}", 1,2,3, idx);
                        }
                        //println!("consume time is : {}", d_now.elapsed().as_millis());

                    }));
        }
    }
}
