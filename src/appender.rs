pub mod file_appender;

pub use self::file_appender::FileAppender;

pub trait Appender: Send {
    fn append(&mut self, log: &String, flush: bool);
}

