use time;
use crate::filter::FilterLevel;
use std::fmt;

use log::Record;

pub struct LogicEvent {
    pub level: FilterLevel,
    pub content: String,
}

pub struct Event {
    pub time_spec: time::Timespec,
    pub tm: time::Tm,
    pub level: FilterLevel,
    pub thread_tag: String,
    pub file: String,
    pub line: u32,
    pub msg: String,
}

impl Event {
    pub fn new(level: FilterLevel, thread_tag: String, file: &'static str, line: u32, msg: fmt::Arguments) -> Self {
        Self {
            time_spec: time::get_time(),
            tm: time::now(),
            level,
            thread_tag,
            file: file.to_string(),
            line,
            msg: msg.to_string(),
        }
    }

    pub fn from_record(thread_tag: String, record: &Record) -> Self {
        Self {
            time_spec: time::get_time(),
            tm: time::now(),
            level: record.level().into(),
            thread_tag,
            file: record.file().unwrap_or("").to_string(),
            line: record.line().unwrap_or(0),
            msg: format!("{}", record.args()),
        }
    }

    pub fn format_by_default(&self) -> String {
        let t = self.tm.strftime("[%Y-%m-%d %H:%M:%S]").unwrap();
        format!("{}-{}-[{}]-{}:{}  {}\n", t, self.thread_tag, self.level.to_str(), self.file, self.line, self.msg)
    }

    pub fn to_logic(self) -> LogicEvent {
        LogicEvent {
            content: self.format_by_default(),
            level: self.level,
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "(tm:{:?} level:{} thread_tag:{} file:{} line:{} msg:{})",
               self.tm,
               self.level.to_str(),
               self.thread_tag,
               self.file,
               self.line,
               self.msg
        )
    }
}

