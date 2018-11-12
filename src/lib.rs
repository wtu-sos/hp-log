#![feature(duration_as_u128)]
//extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate lazy_static;


mod event;
mod config;
mod writer;
mod appender;

pub mod filter;
pub mod logger;

//log_info, log_debug, log_error, log_warn, log_fatal
pub use crate::{
    logger::{send_event, Logger},
};

mod test {

    #[test]
    fn multi_test() {
        //use std::time::{Duration, Instant};
        use std::thread;
        use std::path::PathBuf;
        //use std::io;
        use crate::{send_event, Logger, log_info, log_debug, log_error};

        Logger::load_config(PathBuf::from("./"));

        let mut ths = Vec::new();

        let max_format_count = 50_0000;
        let t1 = max_format_count.clone(); 

        //let m_now = Instant::now();
        for _i in 0..8 {
            ths.push(thread::spawn(
                    move || {

                        //let d_now = Instant::now();
                        for idx in 0..t1 {
                            log_debug!("1234567890-=dfghjkl;'kald;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m .... {}", 1,2,3, idx);
                            log_error!("1234567890-=dfghjkl;'kald;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m ********* {}", 1,2,3, idx);
                            log_info!("1234567890-=dfghjkl;'ka0000000000;ngtohbjgbtesting {}...99=====.{}....mmmmmmm{}m ********* {}", 1,2,3, idx);
                        }
                        //println!("consume time is : {}", d_now.elapsed().as_millis());

                    }));
        }
    }
}
