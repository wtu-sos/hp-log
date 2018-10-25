use std::sync::mpsc;
pub struct Filter {
}

struct Logger {
    appenders: Vec<Appender>,
    sender: mpsc::Sender,
}

//#[macro_export]
//macro_rules! log_debug {
//    ($($arg:tt)*) => {
//        $crate::logger_core::with_local(|local| {
//            if $crate::filter::filter($crate::filter::DEBUG, local.filter) { 
//                local.send_event($crate::filter::DEBUG, file!(), line!(), format_args!($($arg)*));
//            }})
//    }
//}


