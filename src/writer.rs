use std::collections::LinkedList;
use std::sync::{Arc, Mutex, Condvar};
use crate::appender::Appender;

pub struct Writer {
    inner: Arc<Inner>,
    event_cache: LinkedList<String>,
    appenders: Vec<Box<Appender>>,
}

pub struct Inner {
    cond: Condvar,
    events: Mutex<LinkedList<String>>,
}

pub struct Poster {
    inner: Arc<Inner>,
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

        self.cond.notify_one();
    }
}

impl Poster {
    #[allow(dead_code)]
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
            appenders: Vec::new(),
        }
    }

    pub fn get_poster(&self) -> Poster {
        Poster {
            inner: self.inner.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn add_appender(&mut self, appender: Box<Appender>) {
        self.appenders.push(appender);
    }

    #[allow(dead_code)]
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
            //println!("append logs, cache size: {}", self.event_cache.len());
        }

        while !self.event_cache.is_empty() {
            match self.event_cache.pop_front() {
                Some(log) => {
                    for app in self.appenders.iter_mut() {
                        app.append(&log, self.event_cache.is_empty());
                    }
                },
                None => {
                    println!("Error cache empty!");
                    break;
                }
            }
        }
    }
}

