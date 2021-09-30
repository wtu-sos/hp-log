use std::fs::{self, File};
use std::fs::{OpenOptions, DirBuilder};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::event::LogicEvent;
use crate::appender::Appender;
use crate::filter::Filters;
use crate::config::FilterConf;

pub struct FileAppender {
    filter: Filters,
    max_buf: usize,
    buf: String,
    log_max_size: u64,
    roll_index: u32,
    base_dir: String,
    file_name: String,
    file: Option<File>,
}

impl FileAppender {
    pub fn new(conf: FilterConf, buf_size: usize, base_dir: String) -> Self {
        // todo : check current index
        let _ = DirBuilder::new().recursive(true).create(base_dir.clone());
        let log_name = FileAppender::get_file_name(&base_dir, 0);
        //println!("base dir: {:?}, log_name:{:?}", base_dir, log_name);
        let file = OpenOptions::new().create(true)
                                    .write(true)
                                    .append(true)
                                    .open(log_name.clone())
                                    .unwrap_or_else(|_| panic!("open or create file error: {}", log_name));

        Self {
            filter: Filters::new(conf),
            max_buf: buf_size,
            buf: String::with_capacity(buf_size),
            log_max_size: 1024u64*1024u64*512,
            roll_index: 0,
            base_dir,
            file_name: log_name,
            file: Some(file),
        }
    }

    fn get_file_name(dir_name: &str, idx: u32) -> String {
        let exec_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("/tmp/unknow_file"));
        let exe = exec_path.as_path().file_name().expect("get exe path failed");
        //println!("exec_path: {:?}, exe:{:?}, dir_name:{:?}", exec_path, exe, dir_name);
        let p = Path::new(dir_name).join(exe.to_str().unwrap());
        //println!("path: {}", p.display());
        //println!("path: {:?}, file: {:?}, process_id: {}", p.parent(), p.file_name(), std::process::id());
        let log_file_name = format!("{}-{}-{}.log", p.display(), std::process::id(), idx);
        println!("new file name is: {}", log_file_name);

        log_file_name
    }

    pub fn roll_file(&mut self) {
        match fs::metadata(self.file_name.clone()) {
            Ok(metadata) => {
                //println!("file len : {}", metadata.len());
                //println!("file modify time : {:?}", metadata.modified());
                if metadata.len() > self.log_max_size {
                    self.roll_index += 1;
                    self.file_name = FileAppender::get_file_name(&self.base_dir, self.roll_index);
                    let file = OpenOptions::new().create(true)
                                                .write(true)
                                                .append(true)
                                                .open(self.file_name.clone())
                                                .unwrap_or_else(|_| panic!("open or create file error: {}", self.file_name));
                    self.file = Some(file);
                }
            },
            Err(e) => {
                panic!("error occur in get metadata of {}, e: {:?}", self.file_name, e);
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

                r.map_err(|e| e.to_string())
            },
            None => {
                panic!("file is not exist!");
            }
        }

    }
}

impl Appender for FileAppender {
    fn append(&mut self, log: &LogicEvent, flush: bool) {
        if !self.filter.is_pass(log.level) {
            return;
        }

        if self.buf.len() + log.content.len() > self.max_buf && self.write_file().is_err() {
            panic!("there is something wrong while write log file");
        }

        self.buf.push_str(log.content.as_str());

        if flush &&  self.write_file().is_err() {
            panic!("there is something wrong while write log file");
        }
    }

    fn flush(&mut self) {
        self.write_file();
    }
}
