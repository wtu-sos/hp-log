use writer::{Poster, Writer, FileAppender};
use event::Event;
use std::thread;
use std::sync::mpsc;

pub struct Filter {
}

pub enum EventType {
    Log(Event),
}

struct Logger {
    appenders: Vec<FileAppender>,
    sender: mpsc::Sender,
    wr_th: thread::JoinHandle,
    fmt_th: thread::JoinHandle,
    poster: mpsc::Sender<EventType>,
}

impl Logger {
    pub fn init() -> Self {
        let mut w = Writer::new();
        let poster = w.get_poster();

        // init writer thread
        let wr_th = thread::spawn(move ||{
            loop {
                w.fetch_logs();

                // release cpu every frame 
                // todo : to be more intelligent
                thread::sleep_ms(1u32);
            }
        });

        let (tx, rx) = mpsc::channel();
        let fmt_th = thread::spawn(move || {
            loop {
                for msg in rx {
                    // todo : handle msg 
                    match msg {
                        Log(log) => {
                            poster.insert_log(log.format_by_default());
                        }
                    }
                }

                // release cpu every frame 
                // todo : to be more intelligent
                thread::sleep_ms(1u32);
            }
        });
    }    
}

//#[macro_export]
//macro_rules! log_debug {
//    ($($arg:tt)*) => {
//        $crate::logger_core::with_local(|local| {
//            if $crate::filter::filter($crate::filter::DEBUG, local.filter) { 
//                local.send_event($crate::filter::DEBUG, file!(), line!(), format_args!($($arg)*));
//            }})
//    }
//}


