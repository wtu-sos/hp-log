use crate::config::FilterConf;
use crate::color::Color;

#[derive(Copy, Clone)]
pub enum FilterLevel {
    Debug = 1,
    Info  = 2,
    Warn  = 4,
    Error = 8,
    Fatal = 16,
} 

pub struct Filters {
    filter: u8,
}


#[allow(dead_code)]
impl Filters {
    pub fn new(conf: &FilterConf) -> Self {
        //println!("conf : {:?}", conf);
        let mut filter = 0u8;
        if conf.debug {
            filter |= FilterLevel::Debug as u8;
        }
        if conf.info {
            filter |= FilterLevel::Info as u8;
        }

        if conf.warn {
            filter |= FilterLevel::Warn as u8;
        }

        if conf.error {
            filter |= FilterLevel::Error as u8;
        }

        if conf.fatal {
            filter |= FilterLevel::Fatal as u8;
        }

        //println!("filter : {}", filter);

        Filters {
            filter,
        }
    }

    pub fn get_filter(&self) -> u8 {
        self.filter
    }

    #[allow(dead_code)]
    pub fn is_pass(&self, level: FilterLevel) -> bool {
        return 0 != (self.filter & level as u8);
    }
}

#[allow(dead_code)]
impl FilterLevel {
    pub fn to_str(&self) -> &'static str {
        match self {
            FilterLevel::Debug => "DEBUG",
            FilterLevel::Info  => "INFO",
            FilterLevel::Warn  => "WARN",
            FilterLevel::Error => "ERROR",
            FilterLevel::Fatal => "FATAL",
        } 
    }

    pub fn fg_color(&self) -> Color {
        match self {
            FilterLevel::Debug => Color::Cyan,
            FilterLevel::Info  => Color::Green,
            FilterLevel::Warn  => Color::Yellow,
            FilterLevel::Error => Color::Red,
            FilterLevel::Fatal => Color::Red,
        } 
    }

    pub fn bg_color(&self) -> Color {
        match self {
            FilterLevel::Debug => Color::Black,
            FilterLevel::Info  => Color::Black,
            FilterLevel::Warn  => Color::Black,
            FilterLevel::Error => Color::Black,
            FilterLevel::Fatal => Color::White,
        } 
    }

    pub fn from(digit: u8) -> FilterLevel {
        match digit {
            1  => FilterLevel::Debug,
            2  => FilterLevel::Info,
            4  => FilterLevel::Warn,
            8  => FilterLevel::Error,
            16 => FilterLevel::Fatal,
            _ => unimplemented!(),
        }
    }
}
