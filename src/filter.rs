use crate::config::FilterConf;
use crate::color::Color;
use log::Level;

#[derive(Copy, Clone)]
pub enum FilterLevel {
    Off   = 0,
    Debug = 1,
    Info  = 2,
    Warn  = 4,
    Error = 8,
    Trace  = 16,
} 

pub struct Filters {
    filter: u8,
}


#[allow(dead_code)]
impl Filters {
    pub fn new(conf: FilterConf) -> Self {
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

        if conf.trace {
            filter |= FilterLevel::Trace as u8;
        }

        Filters {
            filter,
        }
    }

    pub fn get_filter(&self) -> u8 {
        self.filter
    }

    #[allow(dead_code)]
    pub fn is_pass(&self, level: FilterLevel) -> bool {
        0 != (self.filter & level as u8)
    }

    #[allow(dead_code)]
    pub fn is_enable(&self, level: log::LevelFilter) -> bool {
        0 != (self.filter & level as u8)
    }
}

#[allow(dead_code)]
impl FilterLevel {
    pub fn to_str(self) -> &'static str {
        match self {
            FilterLevel::Debug => "DEBUG",
            FilterLevel::Info  => "INFO",
            FilterLevel::Warn  => "WARN",
            FilterLevel::Error => "ERROR",
            FilterLevel::Trace => "TRACE",
            FilterLevel::Off => "TRACE",
        } 
    }

    pub fn fg_color(self) -> Color {
        match self {
            FilterLevel::Debug => Color::Cyan,
            FilterLevel::Info  => Color::Green,
            FilterLevel::Warn  => Color::Yellow,
            FilterLevel::Error => Color::Red,
            FilterLevel::Trace => Color::White,
            FilterLevel::Off => Color::White,
        } 
    }

    pub fn bg_color(self) -> Color {
        match self {
            FilterLevel::Debug => Color::Black,
            FilterLevel::Info  => Color::Black,
            FilterLevel::Warn  => Color::Black,
            FilterLevel::Error => Color::Black,
            FilterLevel::Trace => Color::Black,
            FilterLevel::Off => Color::Black,
        } 
    }

    pub fn from(digit: u8) -> FilterLevel {
        match digit {
            0  => FilterLevel::Off,
            1  => FilterLevel::Debug,
            2  => FilterLevel::Info,
            4  => FilterLevel::Warn,
            8  => FilterLevel::Error,
            16 => FilterLevel::Trace,
            _ => unimplemented!(),
        }
    }
}

impl From<log::LevelFilter> for FilterLevel {
    fn from(level: log::LevelFilter) -> FilterLevel {
        match level {
            log::LevelFilter::Debug   => FilterLevel::Debug,
            log::LevelFilter::Info    => FilterLevel::Info,
            log::LevelFilter::Warn    => FilterLevel::Warn,
            log::LevelFilter::Error   => FilterLevel::Error,
            log::LevelFilter::Trace   => FilterLevel::Trace,
            _ => FilterLevel::Off,
        }
    }
}
impl From<Level> for FilterLevel {
    fn from(level: Level) -> FilterLevel {
        match level {
            Level::Debug   => FilterLevel::Debug,
            Level::Info    => FilterLevel::Info,
            Level::Warn    => FilterLevel::Warn,
            Level::Error   => FilterLevel::Error,
            Level::Trace   => FilterLevel::Trace,
        }
    }
}
