use std::slice;
use std::str::FromStr;
use std::path::PathBuf;
use std::borrow::Cow;

use crate::{debug, log, Logger};

fn transfer<'a>(content: *const u8, len: usize) -> Cow<'a, str> {
    let arr = unsafe { slice::from_raw_parts(content, len)}; 
    String::from_utf8_lossy(arr)
}

pub extern "C" fn logger_init(content: *const u8, len: usize) {
    let c_str = transfer(content, len);
    let path = PathBuf::from_str(&c_str.into_owned()).unwrap_or(PathBuf::from("./"));
    Logger::load_config(path);
}

pub extern "C" fn debug(content: *const u8, len: usize) {
    let log = transfer(content, len);
    debug!("{}", log);
}
