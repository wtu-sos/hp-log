//extern crate serde;
#[macro_use]
extern crate serde_derive;
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
pub mod cpp;

use std::fmt::Arguments;

pub use crate::{
    logger::{ThreadLocalLogger, Logger, SendEvent},
    event::Event,
    filter::FilterLevel,
};

thread_local! {
    pub static LOG_SENDER: ThreadLocalLogger = ThreadLocalLogger::new();
}

#[allow(unused)]
pub fn send_event(level: FilterLevel, file: &'static str, line: u32, msg: Arguments) {
    LOG_SENDER.with(|s| {
        let ev = Event::new(level, s.get_thread_tag(), file, line, msg);
        s.get_sender().send_event(ev);
    });
}

#[allow(unused)]
#[macro_export]
macro_rules! log {
    ($lv: expr, $arg: expr) => {
        $crate::send_event($lv, file!(), line!(), $arg);
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        log!($crate::filter::FilterLevel::Debug, format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log!($crate::filter::FilterLevel::Info, format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log!($crate::filter::FilterLevel::Error, format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        log!($crate::filter::FilterLevel::Fatal, format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        log!($crate::filter::FilterLevel::Warn, format_args!($($arg)*));
    }
}

mod test {

    #[test]
    fn multi_test() {
        //use std::time::{Duration, Instant};
        use std::thread;
        use std::path::PathBuf;
        //use std::io;
        use crate::{Logger, info, debug, error};

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
                            debug!("1234567890-=dfghjkl;'kald;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m .... {}", 1,2,3, idx);
                            error!("1234567890-=dfghjkl;'kald;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m ********* {}", 1,2,3, idx);
                            info!("1234567890-=dfghjkl;'ka0000000000;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m ********* {}", 1,2,3, idx);
                        }
                        //println!("consume time is : {}", d_now.elapsed().as_millis());

                    }));
        }
    }
}
