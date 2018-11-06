use crate::writer::{Writer};
use crate::event::Event;
use crate::filter::{FilterLevel};
use std::thread;
use std::sync::{mpsc, Mutex};
use std::time::Duration;
use std::fmt::Arguments;


pub enum EventType {
    Log(Event),
}

lazy_static! {
    pub static ref LOGGER_OBJ : Logger = Logger::init();
}

#[allow(unused)]
pub struct Logger {
    wr_th: thread::JoinHandle<()>,
    fmt_th: thread::JoinHandle<()>,
    poster: Mutex<mpsc::Sender<EventType>>,
}

impl Logger {
    pub fn init() -> Self {
        let w = Writer::new();
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
            poster: Mutex::new(tx),
        }
    }    

    pub fn get_poster(&self) -> mpsc::Sender<EventType> {
        self.poster.lock().unwrap().clone()
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        //self.fmt_th.join();
        //self.wr_th.join();
    }
}

pub trait SendEvent {
    fn send_event(&self, e: Event);
}

#[allow(unused)]
impl SendEvent for mpsc::Sender<EventType> {
    fn send_event(&self, e: Event) {
        self.send(EventType::Log(e));
    }
}

pub struct ThreadLocalLogger {
    sender: mpsc::Sender<EventType>,
    thread_tag: String,
} 

impl ThreadLocalLogger {
    pub fn new() -> Self {
        let tid: u64 = unsafe { std::mem::transmute(thread::current().id()) };
        let thread_tag = match thread::current().name() {
            Some(ref name) => format!("{}:[{}]", name.to_string(), tid),
            None => format!("thread:[{}]", tid),
        };

        Self {
            sender: LOGGER_OBJ.get_poster(),
            thread_tag, 
        }
    }

    fn get_thread_tag(&self) -> String {
        self.thread_tag.clone()
    }
}

thread_local! {
    static LOG_SENDER: ThreadLocalLogger = ThreadLocalLogger::new();
}

#[allow(unused)]
pub fn send_event(level: FilterLevel, file: &'static str, line: u32, msg: Arguments) {
    LOG_SENDER.with(|s| {
        let ev = Event::new(level, s.get_thread_tag(), file, line, msg);
        s.sender.send_event(ev);
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
