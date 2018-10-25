#![feature(duration_as_u128)]
extern crate strfmt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use strfmt::strfmt;

use std::collections::HashMap;
use std::time::Instant;
use std::thread;
use std::path::PathBuf;

mod event;
mod config;
mod filter;

use self::event::Event;
use self::filter::Filters;

fn fmt_by_strfmt(map: &HashMap<String, &str>) {
    let _result = format!("{}-{}-{}", "File Name", "1234546", 12345);
    let fmt = "{name}-{job}-{id}!".to_string();
    let _result = strfmt(&fmt, map).unwrap();
    //println!("{}", _result);
}

fn main() {
    config::Config::create_instance(Some(PathBuf::from("./")));

    let filters = Filters::generate_by_config().get_filter();
    println!("filter: {:b}", filters);
    let max_format_count = 100_000;
    let t1 = max_format_count.clone(); 
    let t2 = max_format_count.clone(); 
    let th1 = thread::spawn(move ||{
        let event = Event::new("Debug", 1.to_string(), file!(), line!(), "testing ...99=====.....mmmmmmmm".to_string());

        let d_now = Instant::now();
        for _ in 0..t1 {
            event.format_by_default();
        //    fmt_by_default_macro();
        }
        println!("consume time is : {}", d_now.elapsed().as_millis());
    });
    let th2 = thread::spawn(move ||{
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "File Name");
        vars.insert("job".to_string(), "1234546");
        vars.insert("id".to_string(), "12345");
        let s_now = Instant::now();
        for _ in 0..t2 {
            fmt_by_strfmt(&vars);
        }
        println!("consume time is : {}", s_now.elapsed().as_millis());
    });

    th1.join();
    th2.join();
}
