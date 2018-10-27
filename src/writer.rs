use std::collections::LinkedList;
use std::fs::File;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex, Condvar};
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::thread::{self, JoinHandle};

//use crate::filter::Filters;
//use crate::event::Event;


pub struct Appender {
    //filter: Filters,
    max_buf: usize,
    buf: String,
    file_name: String,
    file: Option<File>,
}

pub struct Writer {
    inner: Arc<Inner>,
    event_cache: LinkedList<String>,
    appenders: Vec<Appender>,
}

pub struct Inner {
    cond: Condvar,
    events: Mutex<LinkedList<String>>,
}

pub struct Poster {
    inner: Arc<Inner>,
}

impl Appender {
    pub fn new(buf_size: usize, file_name: String) -> Self {
        let file = OpenOptions::new().create(true).write(true).open(file_name.clone()).unwrap();
        Appender {
            max_buf: buf_size,
            buf: String::with_capacity(buf_size),
            file_name,
            file: Some(file),
        }
    }

    pub fn append_log(&mut self, log: &String) {
        if self.buf.len() + log.len() > self.max_buf {
            if self.write_file().is_err() {
                panic!("there is something wrong while write log file");
            }
        }

        self.buf.push_str(log);
    }

    pub fn write_file(&mut self) -> Result<(), String>{
        if self.buf.is_empty() {
            return Err(String::from("str empty"));
        }
        match &mut self.file {
            Some(ref mut file) => {
                let r = file.write_all(self.buf.as_bytes());

                //println!("write log : {:?}, result : {:?}", self.buf, r);
                self.buf.clear();
                return r.map_err(|e| e.to_string());
            },
            None => {
                println!("file is not exist!");
                panic!("file is not exist!");
            }
        }
    }
}

impl Inner {
    fn new() -> Self {
        Inner {
            cond: Condvar::new(),
            events: Mutex::new(LinkedList::new()),
        }
    }

    fn insert_log(&self, log: String) {
        let mut lock = self.events.lock().unwrap(); 
        //println!("insert log : {:?}", log);
        lock.push_back(log);

        //let be_notify = if self.event_cache.is_empty() { true } else { false };
        //if be_notify {
        self.cond.notify_one();
        //} 
    }
}

impl Poster {
    pub fn new(inner: Arc<Inner>) -> Poster {
        Poster {
            inner
        }
    }

    pub fn insert_log(&self, log: String) {
        self.inner.insert_log(log);
    }
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            inner: Arc::new(Inner::new()),
            event_cache: LinkedList::new(),
            appenders: vec![Appender::new(1024, String::from("/tmp/log/test_file.log"))],
        }
    }

    pub fn get_poster(&self) -> Poster {
        Poster {
            inner: self.inner.clone(),
        }
    }

    pub fn add_appender(&mut self, appender: Appender) {
        self.appenders.push(appender);
    }

    pub fn insert_log(&self, log: String) {
        self.inner.insert_log(log);
    }
    
    pub fn fetch_logs(&mut self) {
        {
            let mut lock = self.inner.events.lock().unwrap(); 
            while lock.is_empty() {
                lock = self.inner.cond.wait(lock).unwrap();
            }

            self.event_cache.append(&mut lock);
            println!("append logs, cache size: {}", self.event_cache.len());
        }

        while !self.event_cache.is_empty() {
            match self.event_cache.pop_front() {
                Some(log) => {
                    for app in self.appenders.iter_mut() {
                        app.append_log(&log);
                    }
                },
                None => {
                    println!("Error cache empty!");
                    break;
                }
            }
        }

        for app in self.appenders.iter_mut() {
            let _ = app.write_file();
        }
    }
}

