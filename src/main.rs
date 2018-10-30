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

//use std::sync::Arc;

mod event;
mod config;
mod filter;
mod writer;
mod logger;

use self::event::Event;
use self::filter::Filters;
//use self::writer::Writer;
use self::logger::{Logger, SendEvent, send_event};

//fn fmt_by_strfmt(map: &HashMap<String, &str>) {
//    let _result = format!("{}-{}-{}", "File Name", "1234546", 12345);
//    let fmt = "{name}-{job}-{id}!".to_string();
//    let _result = strfmt(&fmt, map).unwrap();
//    //println!("{}", _result);
//}

fn main() {
    config::Config::create_instance(Some(PathBuf::from("./")));
    let filters = Filters::generate_by_config().get_filter();
    println!("filter: {:b}", filters);

    let mut ths = Vec::new();

    let max_format_count = 50_0000;
    let t1 = max_format_count.clone(); 
    let logger = Logger::init();

    for i in 0..8 {
        //let post = logger.get_poster();
        ths.push(thread::spawn(
                move || {

                    let d_now = Instant::now();
                    for _ in 0..t1 {
                        log_debug!("testing {}...99=====.{}....mmmmmmm{}m", 1,2,3);
                        //let event = Event::new("Debug", i.to_string(), file!(), line!(), );
                        //post.send_event(event);
                    }
                    println!("consume time is : {}", d_now.elapsed().as_millis());

                }));
    }
    loop {
        thread::sleep(Duration::from_micros(1u64));
    };

    //th2.join();
}


//let mut vars = HashMap::new();
//vars.insert("name".to_string(), "File Name");
//vars.insert("job".to_string(), "1234546");
//vars.insert("id".to_string(), "12345");
//let s_now = Instant::now();
//for _ in 0..t2 {
//    fmt_by_strfmt(&vars);
//}
//println!("consume time is : {}", s_now.elapsed().as_millis());
