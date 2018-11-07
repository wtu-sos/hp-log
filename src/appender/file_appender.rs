use std::fs::{self, File};
use std::fs::OpenOptions;
use std::io::Write;
use crate::appender::Appender;

pub struct FileAppender {
    //filter: Filters,
    max_buf: usize,
    buf: String,
    log_max_size: u64,
    roll_index: u32,
    base_name: String,
    file_name: String,
    file: Option<File>,
}

impl FileAppender {
    pub fn new(buf_size: usize, base_name: String) -> Self {
        // todo : check current index
        let log_name = FileAppender::get_file_name(&base_name, 0);
        let file = OpenOptions::new().create(true)
                                    .write(true)
                                    .append(true)
                                    .open(log_name.clone())
                                    .unwrap();

        Self {
            max_buf: buf_size,
            buf: String::with_capacity(buf_size),
            log_max_size: 1024u64*1024u64*512,
            roll_index: 0,
            base_name,
            file_name: log_name,
            file: Some(file),
        }
    }

    fn get_file_name(file_name: &String, idx: u32) -> String {
        //println!("path: {:?}, file: {:?}, process_id: {}", p.parent(), p.file_name(), std::process::id());
        let log_file_name = format!("{}-{}-{}.log", file_name, std::process::id(), idx);
        println!("roll files new file name is: {}", log_file_name);

        log_file_name
    }

    pub fn roll_file(&mut self) {
        match fs::metadata(self.file_name.clone()) {
            Ok(metadata) => {
                //println!("file len : {}", metadata.len());
                //println!("file modify time : {:?}", metadata.modified());
                if metadata.len() > self.log_max_size {
                    self.roll_index += 1;
                    self.file_name = FileAppender::get_file_name(&self.base_name, self.roll_index);
                    let file = OpenOptions::new().create(true)
                                                .write(true)
                                                .append(true)
                                                .open(self.file_name.clone())
                                                .unwrap();
                    self.file = Some(file);
                }
            },
            Err(e) => {
                println!("error occur in get metadata of {}, e: {:?}", self.file_name, e);
            }
        }
    }

    fn write_file(&mut self) -> Result<(), String>{
        if self.buf.is_empty() {
            return Err(String::from("str empty"));
        }

        self.roll_file();

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

