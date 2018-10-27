pub trait Appender {
    fn append(&mut self, log: &String);
    fn flush(&mut self) -> Result<(), String>; 
}

