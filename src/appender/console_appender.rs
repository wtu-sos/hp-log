use crate::appender::Appender;

pub struct ConsoleAppender;

impl Appender for ConsoleAppender {
    fn append(&mut self, log: &String, _flush: bool) {
        print!("{}", log);
    }
}
