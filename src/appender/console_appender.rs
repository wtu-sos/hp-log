use crate::appender::Appender;
use crate::filter::FilterLevel;
use crate::event::LogicEvent;
use crate::color::*;

pub struct ConsoleAppender;

impl Appender for ConsoleAppender {
    fn append(&mut self, log: &LogicEvent, _flush: bool) {
        let content = RichContent::new(log.content.as_str(), log.level);
        print!("{}", content);
    }
}
