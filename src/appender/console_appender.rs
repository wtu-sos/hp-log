use crate::appender::Appender;
use crate::filter::Filters;
use crate::event::LogicEvent;
use crate::config::FilterConf;
use crate::color::*;

pub struct ConsoleAppender {
    filter: Filters,
}

impl ConsoleAppender {
    pub fn new(conf: &FilterConf) -> Self {
        Self {
            filter: Filters::new(conf),
        }
    }
}

impl Appender for ConsoleAppender {
    fn append(&mut self, log: &LogicEvent, _flush: bool) {
        if !self.filter.is_pass(log.level) {
            return;
        }

        let content = RichContent::new(log.content.as_str(), log.level);
        print!("{}", content);
    }
}
