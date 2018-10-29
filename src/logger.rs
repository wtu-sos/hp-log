use crate::writer::{Poster, Writer, FileAppender};
use crate::event::Event;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub struct Filter {
}

pub enum EventType {
    Log(Event),
}

pub struct Logger {
    wr_th: thread::JoinHandle<()>,
    fmt_th: thread::JoinHandle<()>,
    poster: mpsc::Sender<EventType>,
    //writer: Writer,
}

impl Logger {
    pub fn init() -> Self {
        let mut w = Writer::new();
        let poster = w.get_poster();

        // init writer thread
        let wr_th = thread::spawn(move ||{
            let mut w = w;
            loop {
                w.fetch_logs();

                // release cpu every frame 
                // todo : to be more intelligent
                thread::sleep(Duration::from_micros(1u64));
            }
        });

        let (tx, rx) = mpsc::channel();
        let fmt_th = thread::spawn(move || {
            loop {
                for msg in rx.iter() {
                    // todo : handle msg 
                    match msg {
                        EventType::Log(log) => {
                            poster.insert_log(log.format_by_default());
                        }
                    }
                }

                // release cpu every frame 
                // todo : to be more intelligent
                thread::sleep(Duration::from_micros(1u64));
            }
        });

        Self {
            wr_th,
            fmt_th,
            poster: tx,
            //writer: w,
        }
    }    

    pub fn get_poster(&self) -> mpsc::Sender<EventType> {
        self.poster.clone()
    }
}

pub trait SendEvent {
    fn send_event(&self, e: Event);
}

impl SendEvent for mpsc::Sender<EventType> {
    fn send_event(&self, e: Event) {
        self.send(EventType::Log(e));
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


