use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use crate::appender::Appender;

pub struct FileAppender {
    //filter: Filters,
    max_buf: usize,
    buf: String,
    file_name: String,
    file: Option<File>,
}

impl FileAppender {
    pub fn new(buf_size: usize, file_name: String) -> Self {
        let file = OpenOptions::new().create(true).write(true).open(file_name.clone()).unwrap();
        Self {
            max_buf: buf_size,
            buf: String::with_capacity(buf_size),
            file_name,
            file: Some(file),
        }
    }

//    pub fn append_log(&mut self, log: &String) {
//        if self.buf.len() + log.len() > self.max_buf {
//            if self.write_file().is_err() {
//                panic!("there is something wrong while write log file");
//            }
//        }
//
//        self.buf.push_str(log);
//    }

    fn write_file(&mut self) -> Result<(), String>{
        if self.buf.is_empty() {
            return Err(String::from("str empty"));
        }

        match &mut self.file {
            Some(ref mut file) => {
                let r = file.write_all(self.buf.as_bytes());

                //println!("write log : {:?}, result : {:?}", self.buf, r);
                self.buf.clear();
                return r.map_err(|e| e.to_string());
            },
            None => {
                println!("file is not exist!");
                panic!("file is not exist!");
            }
        }
    }
}

impl Appender for FileAppender {
    fn append(&mut self, log: &String, flush: bool) {
        if self.buf.len() + log.len() > self.max_buf {
            if self.write_file().is_err() {
                panic!("there is something wrong while write log file");
            }
        }

        self.buf.push_str(log);

        if flush {
            if self.write_file().is_err() {
                panic!("there is something wrong while write log file");
            }
        }
    }
}

