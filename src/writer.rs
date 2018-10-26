use std::collections::LinkedList;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};

//use crate::filter::Filters;
use crate::event::Event;

pub struct Appender {
    //filter: Filters,
    buf: String,
    file_name: Box<Path>,
    file: Option<File>,
}

pub struct Writer {
    events: Arc<Mutex<LinkedList<Event>>>,
    appenders: Vec<Appender>,
}
