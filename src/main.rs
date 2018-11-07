#![feature(duration_as_u128)]
extern crate strfmt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate lazy_static;

//use strfmt::strfmt;

//use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread;
use std::path::PathBuf;
use std::io;

//use std::sync::Arc;

mod event;
mod config;
mod filter;
mod writer;
mod logger;
mod appender;

//use self::event::Event;
//use self::writer::Writer;
//use self::filter::Filters;
use self::logger::{send_event};

fn main() {
    config::Config::create_instance(Some(PathBuf::from("./")));
    //let filters = Filters::generate_by_config().get_filter();

    let mut ths = Vec::new();

    let max_format_count = 50_0000;
    let t1 = max_format_count.clone(); 

    let m_now = Instant::now();
    for _i in 0..8 {
        //let post = logger.get_poster();
        ths.push(thread::spawn(
                move || {

                    let d_now = Instant::now();
                    for _ in 0..t1 {
                        log_debug!("1234567890-=dfghjkl;'kald;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m", 1,2,3);
                        log_error!("1234567890-=dfghjkl;'kald;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m", 1,2,3);
                    }
                    println!("consume time is : {}", d_now.elapsed().as_millis());

                }));
    }
    loop {
        let mut buf = String::new();
        let input = io::stdin().read_line(&mut buf);
        if input.is_ok() {
            println!("consume time is : {} msec", m_now.elapsed().as_millis());
        }
        thread::sleep(Duration::from_micros(1u64));
    };

    //th2.join();
}

