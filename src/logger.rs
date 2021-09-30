use crate::writer::Writer;
use crate::event::Event;
use crate::config::Config;
use crate::appender::{FileAppender, ConsoleAppender};

use std::thread;
use std::sync::{mpsc, Mutex};

pub enum EventType {
    Log(Event),
    Terminate,
}

lazy_static! {
    pub static ref LOGGER_OBJ : Logger = Logger::init();
}

#[allow(unused)]
pub struct Logger {
    wr_th: Mutex<Option<thread::JoinHandle<()>>>,
    fmt_th: Mutex<Option<thread::JoinHandle<()>>>,
    poster: Mutex<mpsc::Sender<EventType>>,
}

impl Logger {
    pub fn init() -> Self {
        let mut w = Writer::new();
        let poster = w.get_poster();

        let console_conf = Config::instance().console_conf();
        println!("{:?}", console_conf);
        if console_conf.switch {
            w.add_appender(Box::new(ConsoleAppender::new(console_conf)));
        }

        let file_conf = Config::instance().file_conf();
        if file_conf.switch {
            w.add_appender(Box::new(FileAppender::new(file_conf, Config::instance().file_temp_buf(), Config::instance().file_log_dir())));
        }

        // init writer thread
        let wr_th = thread::spawn(move ||{
            let mut w = w;
            loop {
                w.fetch_logs();

                if w.is_terminate() {
                    //println!("writer thread exit!!");
                    w.flush_all();
                    break;
                }
                // release cpu every frame 
                // todo : to be more intelligent
                //thread::sleep(Duration::from_micros(1u64));
                thread::yield_now();
            }
        });

        let (tx, rx) = mpsc::channel();
        let fmt_th = thread::spawn(move || {
            let mut is_stop: bool = false;
            loop {
                for msg in rx.iter() {
                    // todo : handle msg 
                    match msg {
                        EventType::Log(log) => {
                            poster.insert_log(log.to_logic());
                        },
                        EventType::Terminate => {
                            is_stop = true;
                            break;
                        },
                    }
                }

                if is_stop {
                    //println!("farmat thread exit!!");
                    poster.set_terminate(true);
                    break;
                }

                // release cpu every frame 
                // todo : to be more intelligent
                //thread::sleep(Duration::from_micros(1u64));
                thread::yield_now();
            }
        });

        Self {
            wr_th: Mutex::new(Some(wr_th)),
            fmt_th: Mutex::new(Some(fmt_th)),
            poster: Mutex::new(tx),
        }
    }    

    pub fn get_poster(&self) -> mpsc::Sender<EventType> {
        self.poster.lock().unwrap().clone()
    }

    pub fn close() {
        //println!("logger closing");
        if let Err(e) = LOGGER_OBJ.get_poster().send(EventType::Terminate) {
            panic!("can not send terminate info to log threads! error: {}", e.to_string());
        }
        if let Some(w_th) = LOGGER_OBJ.wr_th.lock().expect("get write thread failed").take() {
            w_th.join().expect("Couldn't join on the associated thread");
        }
        if let Some(fmt_th) = LOGGER_OBJ.fmt_th.lock().expect("get format thread failed").take() {
            fmt_th.join().expect("Couldn't join on the associated thread");
        }
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

    pub fn get_thread_tag(&self) -> String {
        self.thread_tag.clone()
    }

    pub fn get_sender(&self) -> &mpsc::Sender<EventType> {
        &self.sender
    }

}

impl Default for ThreadLocalLogger {
    fn default() -> Self {
        Self::new()
    }
}