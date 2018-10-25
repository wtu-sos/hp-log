use time;
use std::fmt;

pub struct Event {
    pub time_spec: time::Timespec,
    pub tm: time::Tm,
    pub level: &'static str,
    pub thread_tag: String,
    pub file: &'static str,
    pub line: u32,
    pub msg: String,
}

impl Event {
    pub fn new(level: &'static str, thread_tag: String, file: &'static str, line: u32, msg: String) -> Self {
        Self {
            time_spec: time::get_time(),
            tm: time::now(),
            level,
            thread_tag,
            file,
            line,
            msg,
        }
    }

    pub fn format_by_default(&self) -> String {
        let t = self.tm.strftime("%Y%m%d-%H:%M:%S").unwrap();
        format!("{}-{}-{}:{}  {}", t, self.level, self.file, self.line, self.msg)
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "(tm:{:?} level:{} thread_tag:{} file:{} line:{} msg:{})",
               self.tm,
               self.level,
               self.thread_tag,
               self.file,
               self.line,
               self.msg
        )
    }
}

