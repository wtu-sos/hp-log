use crate::appender::Appender;
use crate::filter::FilterLevel;
use crate::color::*;

pub struct ConsoleAppender;

impl Appender for ConsoleAppender {
    fn append(&mut self, log: &String, _flush: bool) {
        let content = RichContent::new(log, FilterLevel::Debug);
        print!("{}", content);
    }
}
